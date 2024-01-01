class CustomElement {
      /**
      * 
      * if needed, refresh the component
      * @param {string} key the attribute name
      * @param {string} value the attribute value
      */
      setAttribute(key, value){}
      /**
       * it doesn't refresh the component
       * @param {string} key the attribute name
       * @param {string} value the attribute value 
       * @example
      ```javascript
      const component = Sticker.append('my-component');

      //doesn't change anything in the page
      component.setAttributeWithoutRefreshing(key1, value1);
      //doesn't change anything in the page
      component.setAttributeWithoutRefreshing(key2, value2);

      //shows the effect of the 2 changes now!!!
      component.refresh();
      ```
       */
      setAttributeWithoutRefreshing(key, value){}
      /**
       * @see {@link setAttributeWithoutRefreshing}
       */
      refresh(){}
      getElementsByClassName(className){}
      getElementById(id){}
      querySelector(selector){}
}
export default class Sticker {

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
            customElements.define(`${name}-component`, class extends HTMLElement {
                  #text = template.innerHTML;
                  #shadow;
                  #wrapper = document.createElement('div');
                  #attributes = new Map();
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
                  connectedCallback(){
                        this.#shadow = this.attachShadow({mode: 'open'});
                        this.#shadow.append(this.#wrapper);
                        this.#wrapper.innerHTML = this.#text;
                  }
                  getElementsByClassName(className){
                        return this.#shadow.getElementsByClassName(className);
                  }
                  getElementById(id){
                        return this.#shadow.getElementById(id);
                  }
                  querySelector(selector){
                        return this.#shadow.querySelector(selector);
                  }
            }, { extends: 'div'});
            const elem = document.createElement(`${name}-component`);
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
       * @returns {CustomElement} 
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
       * @returns {CustomElement}
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
       * if condition is true, append custom element
       * @param {string} name name of the component
       * @param { boolean } condition condition to check
       * @param {HTMLElement} node
       */
      static if( name, condition, node = document.body ){
            if( condition ){
                  this.append( name, node );
            }
      }
      /**
       * 
       * if condition is true, append custom first specified custom element, else append second custom element
       * @param {string} ifName name of the component appended if condition is true
       * @param {string} elseName name of the component appended if condition is false
       * @param { boolean } condition condition to check
       * @param {HTMLElement} node
       */
      static ifElse( ifName, elseName, condition, node = document.body ){
            if( condition ){
                  this.append( ifName, node );
            }else{
                  this.append( elseName, node );
            }
      }
}

export class SRouter {
      /**
       * @type HTMLDivElement
       */
      static #app;
      /**
      * @type Record<string,string>
       */
      static #routes = {};

      static #enterCallbacks = {};

      static #leaveCallbacks = {};

      static #currentPage = '';

      static #root;

      static get root(){
            return this.#root;
      }
      static set root(value){}
      /**
       * @hideconstructor
       */
      constructor(){};

      /** 
      * creates the router
      * @param {HTMLElement} root where the routes will be displayed. default is document body
      */
      static create( root = document.body ){
            this.#app = document.createElement( 'div' );
            if( !this.#app )
                  throw "cannot create the app router";
            root.append( this.#app );
      }

      /**
       * 
       * @param {Record<string,string>} routes contains all routes of the app. The object has keys that are the name of the route and the values are the actual components used to represent the page
       * @example 
      project structure\
      -home.html\
      -about.html\
      -main.html\
      -script.js\
      -sticker.js
      
      in main.html
      ```
      <sticker>
      #use home.html as home dynamic;
      #use about.html as about dynamic;
      </sticker>
      <script src="script.js" type="module"></script>
      ```
      ...
      in script.js
      ...

      ```javascript
      import { SRouter } from './sticker.js';
      const routes = {
            '/home' : 'home',
            '/about' : 'about'
      };
      SRouter.map( routes );
      ```
       */
      static map( routes ){
            if( typeof routes !== 'object' )
                  throw `cannot use routes because are not of type Record<string,string>`;
            for( let [k,v] of Object.entries(routes) ){
                  if( typeof v !== 'string' || typeof k !== 'string' ){
                        console.warn(`route ${k} not added because it or the component name are not of type string`);
                        continue;
                  }
                  this.#routes[k] = v;
            }
      }
      /**
       * add a route to the routes registry. if routes already exists, it will be overwritten.
       * @param {string} route name
       * @param {string} componentName saved in component registry as dynamic component.
       */
      static add( route, componentName ){
            if( typeof route !== 'string' || typeof componentName !== 'string' ){
                  console.warn(`route ${route} not added because it or the component name are not of type string`);
                  return;
            }
            this.#routes[route] = componentName;
      }
      /**
       * delete given route
       * @param {string} route 
       */
      static delete( route ) {
            if( typeof route !== 'string' || !( route in this.#routes ) )
                  return;
            delete this.#routes[ route ];
      }
      /**
       * go to the specified route
       * @param {string} route 
       */
      static goto( route ){
            if( typeof route !== 'string' || !( route in this.#routes ) ){
                  console.error( `Invalid route. route ${route} does not exist` )
                  return;
            }

            if( this.#currentPage in this.#leaveCallbacks )
                  this.#leaveCallbacks[this.#currentPage]();

            this.#app.innerHTML = '';
            this.#root = Sticker.append( this.#routes[ route ], this.#app );
            
            this.#currentPage = route;
            if( route in this.#enterCallbacks )
                  this.#enterCallbacks[route]();
      }

      static onPageEnter( route, callback ){
            if( typeof route !== 'string' || !( route in this.#routes ) || typeof callback !== 'function' ){
                  console.warn(`route ${route} does not exist or callback is not a function`);
                  return;
            }
            this.#enterCallbacks[route] = callback;
      }

      static onPageLeave( route, callback ){
            if( typeof route !== 'string' || !( route in this.#routes ) || typeof callback !== 'function' ){
                  console.warn(`route ${route} does not exist or callback is not a function`);
                  return;
            }
            this.#leaveCallbacks[route] = callback;
      }
}