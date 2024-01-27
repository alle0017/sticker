# sticker 🦙
Simple html template engine for component syntax and web-component library not based on jsx. i suggest to use "inline html" on vscode as extension to improve dev experience.

# WHY STICKER?

Sticker is simple, fast and lightweight library that can be used to access browser-builtin functions to a lower level than other frameworks. More than that, the sticker template engine enable no javascript site creation with more readable source code. Another cool feature of sticker is that is all open source, so you can easily read all the code and use it for whatever you like.

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

download the repo and run the following command (after installing cargo-cli):
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
As mentioned above, sticker is also javascript friendly, so you can create your own components from js, defining them as templates string (as already mentioned, use 'inline html' extension on vs code to enable better dev experience) rather than jsx. You can also reuse your components created by the template engine, simply adding the 'dynamic' keyword in the component declaration. Some examples and prototypes:

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
/**
* create a prompt like form with an input. the promise resolves when the x is pressed (the prompt is closed) or when the form is submitted
 */
async s.ui.ask(text, placeholder = 'insert here', value = placeholder);
/**
* set an element as draggable
 */
s.ui.draggable(element)
/**
* create title bar for your components. it is based on apple and windows ones. callback function will trigger when the x button is clicked
*/ 
s.ui.titleBar(callback, node = document.body, title = 'window')
```

## UTILS

a set of utils methods that can be used for various purposes

```typescript
import * as s from './sticker/index.js';
enum browser {
      FIREFOX = 'Firefox',
      CHROME = 'Chrome',
      IE = "Trident", 
      MS_IE = "MSIE",
      OPERA = "Opera", 
      SAFARI = "Safari",
      IPHONE_SAFARI = "iPhone",
      IPAD_SAFARI = "iPad",
      WEBKIT = 'AppleWebKit',
      GECKO = 'Gecko',
      MS_EDGE = 'Edge',
      CHROMIUM_EDGE = 'Edg/',
      NINTENDO = ' NintendoBrowser',
};

enum os {
      NINTENDO = 'Nintendo',
      PLAYSTATION = 'PlayStation',
      XBOX = 'XBox',
      ANDROID = 'Android',
      IPHONE = 'iPhone',
      IPAD = 'iPad',
      WINDOWS_PHONE = 'Windows Phone',
      MAC = 'Mac',
      WINDOWS = 'Win',
      UNIX = 'X11',
      CHROMECAST = 'CrKey',
      ROKU = 'Roku',
      LINUX = 'Linux',
      UNKNOWN = 'unknown',
};
//use as follow:
s.utils.os.NINTENDO
s.utils.browser.NINTENDO

/**
 * return the actual os used
 */
s.utils.detectOs(): os;
/**
 * return the actual browser used
 */
s.utils.detectBrowser(): browser;
/**
* return the browser version
 */
s.utils.detectBrowserVersion(): number;
/**
 * @see https://developer.mozilla.org/en-US/docs/Web/API/Navigator/userAgentData for more information about compatibility
 * @returns if device is mobile
 */
s.utils.isMobile(): boolean;
/**
* @returns browser, browserVersion and os
 */
s.utils.getUserAgentInfo();

```
      
