export class Sticker {
      static #customElements = new Map();
      /**
       * @hideconstructor
       */
      constructor(){};
      static #defineComponent(name){
            const template = document.getElementById(name);
            if( !template ){
                  console.warn(`component ${name} doesn't exist`);
                  return;
            }
            customElements.define(`${name}-component`, class extends HTMLDivElement {
                  #text = template.innerHTML;
                  #shadow;
                  #wrapper = document.createElement('div');
                  #attributes = new Map();
                  constructor(attributes = []){
                        super();
                        this.#shadow = this.attachShadow({mode: 'open'});
                        this.#shadow.append(this.#wrapper);
                        this.#wrapper.innerHTML = this.#text;
                  }
                  #serialize(){
                        let text = this.#text;
                        for( let [key, value] of this.#attributes.entries() ){
                              text = text.replace(`{{${key}}}`, value);
                        }
                        this.#wrapper.innerHTML = text;
                  }
                  /**
                   * 
                   * @param {string} key the attribute name
                   * @param {string} value the attribute value
                   * if needed, refresh the component
                   */
                  setAttribute(key, value){
                        const attrib = this.#attributes.get(key);
                        if( !attrib || attrib != value ){
                              this.#attributes.set( key, value );
                              this.#serialize();
                        }
                  }
                  setAttributeWithoutRefreshing(key, value){
                        const attrib = this.#attributes.get(key);
                        if( !attrib || attrib != value ){
                              this.#attributes.set( key, value );
                        }
                  }
                  refresh(){
                        this.#serialize();
                  }
            }, { extends: 'div'});
            const elem = document.createElement('div', {is: `${name}-component`});
            if( !elem ){
                  console.warn(`something went wrong in component ${name} creation`);
                  return;
            }
            this.#customElements.set(name, elem);
            return elem;
      }
      /**
       * 
       * @param {string} name 
       * @returns {HTMLDivElement} 
       */
      static #createComponent(name){
            if( !this.#customElements.get(name) && !this.#defineComponent(name) ){
                  return;
            }
            return this.#customElements.get(name).cloneNode(true);
      }
      /**
       * 
       * @param {string} name 
       * @param {HTMLElement} node 
       */
      static append(name, node = document.body){
            const elem = this.#createComponent(name);
            node.appendChild(elem);
            return elem;
      }
      /**
       * @param {string} name component name
       * @param {Record<string,string>[]} attributes components attributes
       * @returns {(node: HTMLElement)=>void} function that create element for each element of the attributes array
       */
      static for(name,attributes){
            const create = (attribs, node)=>{
                  const elem = this.#createComponent(name);
                  for( const [key,attrib] of Object.entries(attribs) ){
                        elem.setAttributeWithoutRefreshing(key,attrib);
                  }
                  elem.refresh();
                  node.append(elem);
            }
            return (node = document.body)=>{
                  for( let attribs of attributes ){
                        create(attribs, node);
                  }
            }
      }
      /**
       * 
       * @param {string} name name of the component
       * @param { boolean } condition condition to check
       * @param {HTMLElement} node
       * if condition is true, append custom element
       */
      static if( name, condition, node = document.body ){
            if( condition ){
                  this.append( name, node );
            }
      }
      /**
       * 
       * @param {string} ifName name of the component appended if condition is true
       * @param {string} elseName name of the component appended if condition is false
       * @param { boolean } condition condition to check
       * @param {HTMLElement} node
       * if condition is true, append custom first specified custom element, else append second custom element
       */
      static ifElse( ifName, elseName, condition, node = document.body ){
            if( condition ){
                  this.append( ifName, node );
            }else{
                  this.append( elseName, node );
            }
      }
}
