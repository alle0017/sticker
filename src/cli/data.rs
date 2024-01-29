extern crate colored;
extern crate kuchiki;
extern crate csv;
extern crate html5ever;
extern crate markup5ever;

use csv::ReaderBuilder;

use kuchiki::ElementData;
use kuchiki::NodeDataRef;
use kuchiki::NodeRef;
use kuchiki::traits::TendrilSink;

use markup5ever::QualName;
use markup5ever::ns;
use markup5ever::namespace_url;

use colored::Colorize;
use std::collections::HashMap;

const CSV_TAG: &'static str = "csv";
const DOC_PROP: &'static str = "doc";
const VAR_PROP: &'static str = "var";
const FROM_PROP: &'static str = "from";
const TO_PROP: &'static str = "to";

fn get_csv_as_matrix( path: String )->Vec<Vec<String>>{
      println!("{} {}", "reading csv file".blue(), path);
      let mut content: Vec<Vec<String>> = Vec::new();
      let mut reader = match ReaderBuilder::new().from_path(path.clone()){
            Ok(reader) => reader,
            Err(e) => {
                  println!("{} {} {}", "error while reading file".red().bold(), path, e.to_string().red().bold() );
                  return content;
            }
      };
      for record in reader.records() {
            let rec = match record {
                  Ok(rec) => rec,
                  Err(e) => {
                        println!("{} {} {}", "error while reading file".red().bold(), path, e.to_string().red().bold() );
                        return content;
                  }
            };
            let mut vec = Vec::new();
            for col in rec.iter() {
                  vec.push(col.to_string());
            }
            content.push(vec);
      }
      println!("{}", "csv file parsed successfully".green());
      content
}
fn get_node_attribute(  node:  &NodeDataRef<ElementData>, attr: String ) -> Option<String> {
      for a in node.attributes.borrow().map.iter() {
            if a.0.local.to_string() == attr {
                  return Some(a.1.value.to_string());
            }
      }
      None
}
fn node_to_string( node: &NodeDataRef<ElementData> )-> String {
      let mut template = String::new();
      for c in node.as_node().children() {
            template.push_str(c.to_string().as_str());
      }
      template
}
fn string_to_node(snippet: String) -> Option<NodeRef> {
      let mut snippet_with_anchor = snippet;
      snippet_with_anchor.push_str("</template>");
      snippet_with_anchor.insert_str(0, "<template>");
      let element_dom = kuchiki::parse_fragment(
            QualName::new(None, ns!(html), "div".into()), 
            vec![] 
      ).one(snippet_with_anchor);
      let element = match element_dom.select_first("template"){
            Ok(element) => element.template_contents.clone().unwrap(),
            Err(_) => {
                  println!("{}","Error while parsing the tag...".red());
                  return None;
            }
      };
      Some(element)
}
fn format_var( var_name: String, index: usize )->String {
      let mut formatted = String::new();
      formatted.push_str("{{");
      formatted.push_str(&var_name.clone());
      formatted.push('[');
      formatted.push_str(&index.to_string());
      formatted.push(']');
      formatted.push_str("}}");
      formatted
}
fn replace_csv( node: &NodeDataRef<ElementData>, csv_mat: &Vec<Vec<String>>, var_name: String, from: usize, to: usize ) -> Option<NodeRef>{
      let template = node_to_string(node);
      let mut node_list = String::new();
      let mut i = from;
      let mut to_row = to;
      if csv_mat.len() < to {
            to_row = csv_mat.len();
      }
      while i < to_row {
            let mut j = 0;
            let mut temp = template.clone();
            while j < csv_mat.get(i).unwrap().len() {
                  temp = temp.replace(
                        format_var(var_name.clone(), j).as_str(), 
                        csv_mat.get(i).unwrap().get(j).unwrap() 
                  );
                  j+= 1;
            }
            node_list.push_str(temp.as_str());
            i+=1;
      }
      string_to_node(node_list)
}

fn replace_cvs_tags( dom: &NodeRef ){
      let mut docs: HashMap<String,Vec<Vec<String>>> = HashMap::new();
      for tag in dom.select(CSV_TAG).unwrap() {
            let doc = match get_node_attribute(&tag, DOC_PROP.to_string()) {
                  Some(v) => v,
                  None =>{
                        println!("{} {} {}", "tag discarded because it haven't the".red(), DOC_PROP.to_string().bold(), "property".red() );
                        continue;
                  }
            };
            if docs.get(&doc).is_none() {
                  docs.insert( 
                        doc.clone(), 
                        get_csv_as_matrix(doc.clone())
                  );
            }
            let csv = docs.get(&doc).unwrap();
            let var_name = match get_node_attribute(&tag, VAR_PROP.to_string()) {
                  Some(v) => v,
                  None =>{
                        "row".to_string()
                  }
            };
            let from = match get_node_attribute(&tag, FROM_PROP.to_string()) {
                  Some(v) => v.parse().unwrap(),
                  None => 0
            };
            let to = match get_node_attribute(&tag, TO_PROP.to_string()) {
                  Some(v) => v.parse().unwrap(),
                  None => csv.len(),
            };
            let _ = match replace_csv(&tag, csv, var_name, from, to){
                  Some(node)=>{
                        tag.as_node().insert_before(node);
                        tag.as_node().detach();
                  },
                  None => continue,
            };
      }
}

pub fn get_data_from_file( dom: &mut NodeRef ) {
      replace_cvs_tags(dom);
}