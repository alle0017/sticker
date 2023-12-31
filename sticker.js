class CustomElement extends HTMLElement {
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
      /**
       * method called when the element is added to the DOM
       * @example
      ```javascript
      //extend the component
      Sticker.extend('my-component', {
            onenter(){
                  console.log('hello world');
            }
      })
      //create new instance of it
      Sticker.append('my-component');
      //in the console you can see 'hello world'
      ```
       */
      onenter(){}
}
export default class Sticker {

      static #customElements = new Map();

      /**
       * @hideconstructor
       */
      constructor(){};

      /**
       * 
       * @param {string} name 
       * @param {string} component 
       * @returns {CustomElement | undefined}
       */
      static #defineComponent(name, component){
            customElements.define(`${name}-component`, class extends HTMLElement {
                  #text = component;
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
                  getAttribute(key){
                        return this.#attributes.get(key);
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
                        if( this.onenter && typeof this.onenter == 'function' ){
                              this.onenter();
                        }
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
      static #defineComponentFromTemplate(name){
            const template = document.getElementById(name);
            if( !template ){
                  console.warn(`component ${name} doesn't exist`);
                  return;
            }
            return this.#defineComponent(name, template.innerHTML);
      }
      /**
       * 
       * @param {string} name 
       * @returns {CustomElement | undefined} 
       */
      static #createComponent(name){
            if( !this.#customElements.get(name) && !this.#defineComponentFromTemplate(name) ){
                  return;
            }
            return this.#customElements.get(name).cloneNode(true);
      }
      static #setAttributes(elem, attribs){
            if( typeof attribs !== 'object' || Object.keys(attribs).length == 0 )
                  return;
            for( const [key,attrib] of Object.entries(attribs) ){
                  elem.setAttributeWithoutRefreshing(key,attrib);
            }
            elem.refresh();
      }
      /**
       * create new component that can be used in the page
       * @param {string} name 
       * @param {string} template 
       * @returns {CustomElement | undefined}
       */
      static defineCustomElement(name, template){
            return this.#defineComponent(name, template);
      }
      /**
       * 
       * @param {string} name 
       * @param {HTMLElement} node 
       * @param {Record<string,string>} attributes 
       * @returns {CustomElement | undefined}
       */
      static append(name, attributes = {}, node = document.body){
            const el = this.#createComponent(name);
            if( !el )
                  return;
            if( attributes instanceof HTMLElement ){
                  attributes.appendChild(el);
            }else{
                  node.appendChild(el);
                  this.#setAttributes(el, attributes);
            }
            return el;
      }
      /**
       * @param {string} name component name
       * @param {Record<string,string>[]} attributes components attributes
       * @returns {(node: HTMLElement)=>void} function that create element for each element of the attributes array
       */
      static for(name,attributes){
            const create = ((attribs, node)=>{
                  const elem = this.#createComponent(name);
                  this.#setAttributes(elem, attribs);
                  node.append(elem);
            }).bind(this);
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
      /**
       * extends a component with specified methods and properties
       * @param {string} name of the component to extend
       * @param {Record<string,any>} object the object that contains methods and properties to add to the object
       */
      static extends( name, object ){
            if( !this.#customElements.has( name ) ){
                  if( !this.#createComponent( name ) ){
                        console.error( `Component ${name} does not exist`);
                        return;
                  }
            }
            const constructor = customElements.get( `${name}-component`);
            for( let [k,v] of Object.entries( object ) ){
                  constructor.prototype[k] = v;
            }
      }
}

export class SRouter {
      /**
       * @type HTMLDivElement
       */
      #app;
      /**
      * @type Record<string,string>
       */
      #routes = {};

      #enterCallbacks = {};

      #leaveCallbacks = {};

      #currentPage = '';

      #root;

      get root(){
            return this.#root;
      }
      set root(value){}
      /** 
      * creates the router
      * @param {HTMLElement} root where the routes will be displayed. default is document body
      * @param {Record<string,string>} routes contains all routes of the app. The object has keys that are the name of the route and the values are the actual components used to represent the page
      */
      constructor( routes = {}, root = document.body ){
            this.#app = document.createElement( 'div' );
            if( !this.#app )
                  throw "cannot create the app router";
            root.append( this.#app );
            this.map(routes)
      };

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
      const router = new SRouter();
      router..map( routes );
      ```
       */
      map( routes ){
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
      add( route, componentName ){
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
      delete( route ) {
            if( typeof route !== 'string' || !( route in this.#routes ) )
                  return;
            delete this.#routes[ route ];
      }
      /**
       * go to the specified route
       * @param {string} route 
       */
      goto( route, props = {} ){
            if( typeof route !== 'string' || !( route in this.#routes ) ){
                  console.error( `Invalid route. route ${route} does not exist` )
                  return;
            }

            if( this.#currentPage in this.#leaveCallbacks )
                  this.#leaveCallbacks[this.#currentPage]();

            this.#app.innerHTML = '';
            this.#root = Sticker.append( this.#routes[ route ], props, this.#app );
            
            this.#currentPage = route;
            if( route in this.#enterCallbacks )
                  this.#enterCallbacks[route]();
      }

      onPageEnter( route, callback ){
            if( typeof route !== 'string' || !( route in this.#routes ) || typeof callback !== 'function' ){
                  console.warn(`route ${route} does not exist or callback is not a function`);
                  return;
            }
            this.#enterCallbacks[route] = callback;
      }

      onPageLeave( route, callback ){
            if( typeof route !== 'string' || !( route in this.#routes ) || typeof callback !== 'function' ){
                  console.warn(`route ${route} does not exist or callback is not a function`);
                  return;
            }
            this.#leaveCallbacks[route] = callback;
      }
}