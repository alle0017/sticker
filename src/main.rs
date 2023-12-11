extern crate kuchiki;
extern crate colored;
extern crate html5ever;
extern crate markup5ever;
extern crate markdown;

use kuchiki::NodeRef;
use kuchiki::parse_html;
use kuchiki::traits::TendrilSink;

use markup5ever::QualName;


use colored::Colorize;
use markup5ever::ns;
use markup5ever::namespace_url;

use std::fs::File;
use std::io::prelude::*;
use std::env;

#[derive(Default, Debug)]
struct Attributes {
    name: String,
    value: String,
}
impl Attributes {
    fn new() -> Attributes{
        return Attributes {
            name: String::new(),
            value: String::new(),
        };
    }
}
#[derive(Default, Debug, Clone)]
struct CustomTag {
    name: String,
    path: String,
    file_text: String,
}
impl CustomTag {
    fn new() -> CustomTag {
        return CustomTag {
            name: String::new(),
            path: String::new(),
            file_text: String::new(),
        };
    }
    fn attach_html(&mut self, code: &String){
        let html = code.clone();
        self.file_text = html;
    }
    fn init_tag_from_string(&mut self, declaration: &String) {

        self.path = self.get_tag_file(&declaration);

        if self.path.len() <= 0 {
            println!("tag path not recognized {}", declaration.red());
            return;
        }
        self.name = self.get_tag_name(&declaration);
        if self.name.len() <= 0 {
            println!("tag name not found {}", declaration.red());
            return;
        }
    }
    fn get_tag_file(&self, declaration: &String) -> String {
        let index = declaration.find("use");
        let mut path = String::new();
        if !index.is_none() {
            let string = declaration.get(index.unwrap() + 3..).unwrap();

            for c in string.chars() {
                if ( c == ' ' || c == ';' || c == '\n' ) && path.len() > 0 {
                    break;
                }else if c != ' ' && c != ';' && c != '\n'  {
                    path.push(c);
                }
            }
            println!("found path: {}", path.green());
        }
        path
    }
    fn get_tag_name(&self, declaration: &String) -> String {
        let index = declaration.find("as");
        let mut name = String::new();
        if !index.is_none() {
            let string = declaration.get(index.unwrap() + 2..).unwrap();
            for c in string.chars() {
                if c != ' ' {
                    name.push(c);
                } else if c == ' ' && name.len() > 0 {
                    break;
                } 
            }
            println!("found name: {}", name.green());
        }
        name
    }
    fn get_code_from_file(&mut self){
        
        let mut text = match Dom::get_file(&self.path){
            Ok(text)=> text,
            Err(e) => {
                println!("{}", e.to_string().red());
                "".to_string()
            },
        };
        if !self.path.find(".md").is_none() {
            text = markdown::to_html(text.as_str());
        }
        self.attach_html(&text)
    }
    fn add_attributes<'a>(&'a mut self, attributes: &Vec<Attributes>) -> String {
        if self.file_text.len() <= 0 {
            self.get_code_from_file();
        }
        let mut html = self.file_text.clone();
        for attribute in attributes {
            let mut attribute_name = attribute.name.clone();
            let attribute_value = attribute.value.clone();
            attribute_name.insert_str(0, "{{");
            attribute_name.push_str("}}");
            html = html.replace( &attribute_name, &attribute_value );
        }
        html
    }
    fn as_node(&mut self,  attributes: &Vec<Attributes>) -> Option<NodeRef> {
        let snippet = self.add_attributes(attributes);
        let element_dom = kuchiki::parse_fragment(
            QualName::new(None, ns!(), "div".into()), 
            vec![] 
        ).one(snippet);
        let element = match element_dom.select_first("html"){
            Ok(element) => element,
            Err(_) => {
                println!("{}","Error while parsing the tag...".red());
                return None;
            }
        };
        Some(element.as_node().clone())
    }
}
struct Dom {
    dom: NodeRef,
    tags: Vec<CustomTag>,
}
impl Dom {
    const TAG_NAME: &str = "sticker";
    const COMPONENT_DECLARATION: &str = "#use";

    fn new(file_path: &String) -> Self {
        return Dom {
            dom: Dom::create_html_dom(file_path),
            tags: vec![],
        }
    }
    fn create_html_dom( file_path: &String ) -> NodeRef {
        let file_data = match Dom::get_file(file_path){
            Ok(data) => data,
            Err(e) => panic!("problem reading file {}. error{}",file_path.red(), e.to_string().red()),
        };
        let parser = parse_html().one(file_data);
    
        parser
    }
    fn get_file( name: &String ) -> Result<String, std::io::Error> {
        println!("{}", "reading file...".blue());
        println!("name of file: {}", name.truecolor(255,153,0));
        let mut file = File::open(name)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
    
        println!("{}","file read successfully".green());
    
        Ok(content)
    }    
    fn create_file(&self, new_file_name: &str){
        println!("{}","creating file...".blue());
        let mut bin = match File::create(new_file_name) {
            Ok(file) => file,
            Err(e) => panic!("error while creating file (path: {}) error {}", new_file_name.red(), e.to_string().red())
        };
        match bin.write_all(self.dom.to_string().as_bytes()){
            Err(e) => panic!("error while writing on file (path: {}) error {}", new_file_name.red(), e.to_string().red().bold()),
            _=>println!("{}","file written successfully".green().bold())
        };
    }
}

trait CustomTagParser {
    fn get_components_declarations( text_node: &String ) -> Vec<String>;
    fn get_custom_tags(&mut self, declaration_tag_name: &str );
    fn substitute_custom_tags(&self);
    fn parse(&mut self);
}
impl CustomTagParser for Dom {
    fn get_components_declarations( text_node: &String ) -> Vec<String> {
        let text = text_node.clone();
        let mut declarations: Vec<String> = Vec::new();
        let possible_declarations: Vec<&str> = text.split(';').collect();
        for declaration in possible_declarations {
            if !declaration.find(Dom::COMPONENT_DECLARATION).is_none() {
                declarations.push(declaration.to_string());
            }
        }
            
        declarations
    }
    fn get_custom_tags(&mut self, declaration_tag_name: &str ) {
        println!("{}","get custom tags...".blue());
        let declaration_tag = self.dom.select_first(declaration_tag_name).unwrap();
        let text = declaration_tag.text_contents();
        let declarations = Dom::get_components_declarations(&text);

        for declaration in declarations.iter() {
            let mut tag = CustomTag::new();
            println!("declaration string: {}",declaration.truecolor(255,153,0));
            tag.init_tag_from_string(declaration);
            self.tags.push(tag);
        }
        declaration_tag.as_node().detach();

    }
    fn substitute_custom_tags(&self) {
        for prototype_tag_ref in self.tags.as_slice() {
            let mut prototype_tag = prototype_tag_ref.clone();
            println!("working on {}...", prototype_tag.name.blue());
            for tag in self.dom.select(prototype_tag.name.as_str() ).unwrap() {
    
                let attributes = tag.attributes.borrow();
                let mut values: Vec<Attributes> = Vec::new();
                for (a_name, a_value) in attributes.map.iter() {
                    let mut attrib = Attributes::new();
                    attrib.name = a_name.local.to_string();
                    attrib.value = a_value.value.to_string();
                    values.push( attrib );
                }
                let element = prototype_tag.as_node(&values);
                if element.is_none() {
                    continue;
                }
                for child in element.unwrap().children() {
                    tag.as_node().insert_before(child);
                }
                tag.as_node().detach();
                println!("{}", self.dom.to_string().bright_cyan());
            }
        }
        
    }
    fn parse<'a>(&'a mut self){
        self.get_custom_tags( Dom::TAG_NAME );
        self.substitute_custom_tags();
    }
}

fn get_file_path() -> String {
    let mut file_path: String = String::new();
    let current_dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(e) => {
            println!("{}", e.to_string().red());
            panic!();
        },
    };

    println!("{}", current_dir.to_str().unwrap().bold().on_white().black());
    println!("{}","insert file path from current directory:".bold().on_white().black());
    std::io::stdin().read_line(&mut file_path).unwrap();
    file_path = file_path.replace("\n", "");
    file_path.insert(0, '/');
    file_path.insert_str(0, current_dir.to_str().unwrap());
    file_path
}
fn main() {

    let mut file_path: String;
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        file_path = args[1].to_string();
        let current_dir = match std::env::current_dir() {
            Ok(dir) => dir,
            Err(e) => {
                println!("{}", e.to_string().red());
                panic!();
            },
        };
        file_path.insert(0, '/');
        file_path.insert_str(0, current_dir.to_str().unwrap());
    } else {
        file_path = get_file_path();
    }
    
    println!("file path acquired, {}", file_path.green());
    //let dom = create_html_dom(file_path);
    let mut dom: Dom = Dom::new(&file_path);
    dom.parse();

    if args.len() > 2 {
        file_path = args[2].to_string();
        let current_dir = match std::env::current_dir() {
            Ok(dir) => dir,
            Err(e) => {
                println!("{}", e.to_string().red());
                panic!();
            },
        };
        file_path.insert(0, '/');
        file_path.insert_str(0, current_dir.to_str().unwrap());
    } else {
        file_path = get_file_path();
    }

    dom.create_file(&file_path);
}
