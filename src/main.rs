extern crate kuchiki;
extern crate colored;
extern crate html5ever;
extern crate markup5ever;

use kuchiki::NodeRef;
use kuchiki::parse_html;
use kuchiki::traits::TendrilSink;

use markup5ever::QualName;


use colored::Colorize;
use markup5ever::ns;
use markup5ever::namespace_url;

use std::fs::File;
use std::io::prelude::*;

// the name of tag used to define custom tags
const TAG_NAME: &str = "sticker";
const COMPONENT_DECLARATION: &str = "#use";
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
#[derive(Default, Debug)]
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
        println!("attached html {}", html.cyan());
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
    fn add_attributes(&self, attributes: &Vec<Attributes>) -> String {
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
}

/*
* getting file contents
 */
fn get_html_file( name: &String ) -> Result<String, std::io::Error> {
    println!("{}", "reading file...".blue());
    println!("name of file: {}", name.truecolor(255,153,0));
    let mut file = File::open(name)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    println!("{}","file read successfully".green());

    Ok(content)
}

fn create_html_dom( file_path: String ) -> NodeRef {
    let file_data = match get_html_file(&file_path){
        Ok(data) => data,
        Err(e) => panic!("problem reading file {}. error{}",file_path.red(), e.to_string().red()),
    };
    let parser = parse_html().one(file_data);

    parser
}


fn get_custom_tags( node: &NodeRef, tag_name: &str ) -> Vec<CustomTag> {
    println!("{}","get custom tags...".blue());
    let mut tags: Vec<CustomTag> = Vec::new();
    let declaration_tag = node.select_first(tag_name).unwrap();
    let text = declaration_tag.text_contents();
    let declarations = get_components_declarations(&text);

    for declaration in declarations.iter() {
        let mut tag = CustomTag::new();
        println!("declaration string: {}",declaration.truecolor(255,153,0));
        tag.init_tag_from_string(declaration);
        tags.push(tag);
    }
    declaration_tag.as_node().detach();

    tags
}

fn get_components_declarations( text_node: &String ) -> Vec<String> {
    let text = text_node.clone();
    let mut declarations: Vec<String> = Vec::new();
    let possible_declarations: Vec<&str> = text.split(';').collect();
    for declaration in possible_declarations {
        if !declaration.find(COMPONENT_DECLARATION).is_none() {
            declarations.push(declaration.to_string());
        }
    }
        
    declarations
}
fn substitute_custom_tags( dom: &NodeRef, tags: Vec<CustomTag> ){
    
    for mut prototype_tag in tags {
        match get_html_file(&prototype_tag.path){
            Ok(text)=> prototype_tag.attach_html(&text),
            Err(e) => {
                println!("{}", e.to_string().red());
                return;
            },
        };
        println!("working on {}...", prototype_tag.name.blue());
        for tag in dom.select(prototype_tag.name.as_str() ).unwrap() {

            let attributes = tag.attributes.borrow();
            let mut values: Vec<Attributes> = Vec::new();
            for (a_name, a_value) in attributes.map.iter() {
                let mut attrib = Attributes::new();
                attrib.name = a_name.local.to_string();
                attrib.value = a_value.value.to_string();
                values.push( attrib );
            }
            let snippet = prototype_tag.add_attributes(&values);
            let element_dom = kuchiki::parse_fragment(
                QualName::new(None, ns!(), "div".into()), 
                vec![] 
            ).one(snippet);
            println!("{} {}", "extracted tags".yellow(),element_dom.to_string().yellow());
            let element = match element_dom.select_first("html"){
                Ok(element) => element,
                Err(_) => {
                    println!("{}","Error while parsing the tag...".red());
                    continue;
                }
            };
            for child in element.as_node().children() {
                tag.as_node().insert_after(child);
            }
            tag.as_node().detach();
            println!("{}", dom.to_string().bright_cyan());
        }
    }
}
fn create_file( file_name: &String, dom: &NodeRef ){
    println!("{}","creating file...".blue());
    let mut bin = match File::create(file_name) {
        Ok(file) => file,
        Err(e) => panic!("error while creating file (path: {}) error {}", file_name.red(), e.to_string().red())
    };
    match bin.write_all(dom.to_string().as_bytes()){
        Err(e) => panic!("error while writing on file (path: {}) error {}", file_name.red(), e.to_string().red()),
        _=>println!("{}","file written successfully".green())
    };
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

    let mut file_path: String = get_file_path();

    println!("file path acquired, {}", file_path.green());
    let dom = create_html_dom(file_path);
    let tags = get_custom_tags(&dom, TAG_NAME );
    substitute_custom_tags(&dom, tags);

    file_path = get_file_path();
    create_file(&file_path, &dom);
}
