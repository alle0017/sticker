# sticker 🦙
Simple html compiler for component syntax. Simple static html generation. i suggest to use "inline html" on vscode as extension to improve dev experience.

## BASIC USAGE 

component.html

```html
<div>
  hello world from sticker 🦙
</div>
```

index.html

```html
<sticker>
  #use component.html as component;
</sticker>
<component></component>
```

run the following command
```bash
cargo build --release
sudo mv ./target/release/sticker /usr/local/bin
```
to have access to the cli and run

```bash
sticker
```
to see all the commands available
here are some examples:
```bash
sticker build //build the project
sticker new //initialize new sticker project
sticker comp FILE_TO_COMPILE FILE_COMPILED //compile single file
sticker update //update existing js core in the current folder
```


## USING ATTRIBUTES

component.html

```html
<div>
  hello world from sticker {{name}}
</div>
```

index.html

```html
<sticker>
  #use component.html as component;
</sticker>
<component name="🦙"></component>
```

you can also use .md files as components as if they where normal html components.

## USE DYNAMIC COMPONENTS
 you can also import sticker.js in your project to use dynamic components creation. you can use pre-compiled sticker api to use dynamic components in your js code or you can 
 define a component directly in your js, by declaring a template. suggestion: use inline html exension to enable html syntax highlight in your template declaration.
 ```html
 <div>
  hello {{name}}
</div>
 ```

 ```html
<sticker>
  #use hello.html as hello-world dynamic;
</sticker>
 ```

 ```javascript
 import * as s from './sticker/js/index.js';
s.define({
  name: 'hello-world'
})
let component = s.append('hello-world');
component.setAttribute('name', 'world');
 ```
the function implemented are:
 ```typescript  
 /**
 * create custom component and append it. node is document.body by default, Props is a Record<string, any> object that are set as property of the html element created.
 */
s.append(name: string, props: Object, node: HTMLElement): HTMLElement;
/**
* create custom component.
 */
s.create(name): HTMLElement;
/**
* define custom component ALREADY defined with sticker template engine and return function that specifically creates the component created
P.S.: name must contain '-' character
 */
s.define( descriptor: { 
    name: string, 
    watch: string[] | undefined, 
    props: { 
      onenter: Optional<()=>void>, 
      onleave: Optional<()=>void>, 
      [key: string]: any }, 
    }
): (props: Record<string, any>, node: HTMLElement): HTMLCustomElement | undefined;
/**
* define custom component NOT defined with sticker template engine and return function that specifically creates the component created
P.S.: name must contain '-' character
 */
s.define(descriptor: { 
  template: string, 
  name: string, 
  watch: string[] | undefined, 
  props: { 
    onenter: Optional<()=>void>, 
    onleave: Optional<()=>void>, 
    [key: string]: any 
  }, 
}): (props: Record<string, any>, node: HTMLElement): HTMLCustomElement | undefined;
 ```
## CUSTOM COMPONENTS 
```typescript

export class HTMLCustomElement extends HTMLElement {
      /**
       * 
       * @param {string} name array name used in the html template
       * @param {Array} value 
       */
      setArray(name, value){}
      /**
      * fired when the element enters the DOM
       */
      onenter(){}
      /**
      * fired when the element leaves the DOM
       */
      onleave(){}
      /**
      * @param {string} className
      * @returns { HTMLCollection } live list of elements
       */
      getElementsByClassName(className){}
      /**
      * @param {string} id
      * @returns { HTMLElement } live element
       */
      getElementById(id){}
      /**
      * @param {string} selector
      * @returns { HTMLElement } live element
       */
      querySelector(selector){}
      /**
      * @param {string} selector
      * @returns { HTMLCollection } live list of elements
       */
      querySelectorAll(selector){}
      /**
      * @param {string} selector
      * @returns { HTMLElement } live element
       */
      get(selector){}
      /**
      * @param {string} selector
      * @returns { HTMLCollection } live list of elements
       */
      getAll(selector){}
      /**
      * add new property as watchable property
      * @param {string} propName
       */
      setWatchable(propName) {}
}
```
### FOR SYNTAX
in the template, in s.define, you can also use for attribute to create templates based on arrays
```typescript
s.define({
  ...,
  template: /*html*/`
      <ul for="name of names">
        <li>
          {{name}}
        </li>
      </ul>
  `
  ...
})
...
component.setArray('names', ['John', 'Bob', 'Karl']);
```
## ROUTER
```typescript
import * as s from './sticker/index.js'
/**
* create a new instance of Router and configure the routes with map object as follow: 
key of the object is the name of the route
value of the object key is the component name of the route
@example
{
  '/home': 'home-page'
}
// '/home' is the name of the route, 'home-page' is the component name created with sticker, used to create the page
 */
constructor( map: Record<string,string>, node: HTMLElement = document.body )
/**
change the root where the pages are displayed
 */
router.setRoot( node: HTMLElement )
/**
* change the displayed route and goes to the route named as the parameter
 */
router.goto(route: string)
/**
create new routes as the constructor does
 */
router.map(map: Record<string,string>)
```

## ui

```typescript
s.ui.ask(text, placeholder = 'insert here', value = placeholder);
s.ui.draggable(element)
```
