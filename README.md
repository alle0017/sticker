# sticker 🦙
Simple html template engine for component syntax and web-component library not based on jsx. I suggest to use "inline html" on vs code as extension, to improve dev experience.

# WHY STICKER?

Sticker is simple, fast and lightweight library that can be used to access browser-builtin functions to a lower level than other frameworks. More than that, the sticker template engine enable no javascript site creation with more readable source code. Another cool feature of sticker is that is all open source, so you can easily read all the code and use it for whatever you like.

# INDEX
- [basic usage](#basic-usage)
- [attributes](#using-attributes)
- [md files](#use-md-files)
- [csv files](#use-csv-file)
- [dynamic components](#use-dynamic-components)
- [custom components](#custom-components)
- [for attribute](#for-attribute)
- [ref attribute](#ref-attribute)
- [ui](#ui)
- [utils](#utils)
- [threads](#thread)

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
## USE MD FILES 

you can also use .md files as components as if they where normal html components. it is compiled to html at compile time.
```html
<sticker>
  #use component.md as component;
</sticker>
<component name="🦙"></component>
```

## USE CSV FILE
\
you can also use csv files to generate custom html from templates.
for example, let's say you have a "file.csv" filled with name in the first column and surname in the second column: 
```html
<csv doc="file.csv" from="0" to="20" var="name-and-surname">
    hello name: {{name-and-surname[0]}} surname: {{name-and-surname[1]}}
</csv>
```
the property of the csv tag are:
- doc: the document used as reference
- from (optional, default 0): the row-index to use as starting point
- to (optional, default EOF): the row-index to use as ending point
- var (optional, default row): the name used as template
\
#### ADVERTISEMENT

1) you cannot use the csv file via js, it is compiled and removed at compile time
2) you cannot use tags like table, because the parser will automatically correct it.


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
 \
 or using plain js syntax:
 ```javascript
import * as s from './sticker/js/index.js';
s.define({
  name: 'hello-world'
  template: /*html*/`
        <div>
          hello {{name}}
        </div>
  `,
})
let component = s.append('hello-world');
component.setAttribute('name', 'world');
 ```

the functions implemented for components creation/definition are:
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
### FOR ATTRIBUTE
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
component.setArray('names', ['🦙','John', 'Bob', 'Karl']);
```
### REF ATTRIBUTE
if you want to get an html element in your component, you can, similarly to vueJs, add the attribute ref to the element, than you can refer to it directly in the code as follow:

```typescript
s.define({
  ...,
  template: /*html*/`
      <ul ref="myList">
        <li>
          {{name}}
        </li>
      </ul>
  `
  ...
})
...
component.refs.myList
```
P.S.: this syntax doesn't support already the for loops.

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
* go to the last visited page before the current one
 */
router.back()
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

a set of utils methods that can be used for various purposes. In current state, the utils are methods for browser sniffing.

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

## THREAD

thread provides some utility methods for working with threads in javascript. The threads created with the s.Thread class are not visible to the end user (you will never get an instance of worker) but is handled internally.

```typescript
/**
* create a new thread that will be handled by the s.Thread class
 */
s.Thread.spawn(id: string, code: string | URL)

/**
* true if isn't the main thread
 */
s.Thread.isChildThread()

/**
* listen for the specified message. the callback will get data sended with post or expose as parameter
 */
s.Thread.listen( message: string, callback: Function, id?: string )

/**
* wait for message to be received.
* the messages sended and received will be processed when the message is received.
* available only for one message at time.
*/
s.Thread.wait(message: string, id?: string)

/**
* expose an object to the thread with the given id or to the main thread if the id is omitted. use listen method to capture the message from the thread
 */
s.Thread.post( message: string, data: any, id?: string )

/**
* share with the specified thread the transferable
*/
s.Thread.expose( message: string, transferable: Record<string,Transferable>, id?: string )

/**
* log a message from child thread or main thread
 */
s.Thread.log( message: any )

/**
* log an error from child thread or main thread
 */
s.Thread.error( message: any )

/**
* kill the current thread or the child thread with the specified id 
*/
s.Thread.kill( id: string )

/**
* return promise resolved when the child thread end with the function kill
 */
await s.Thread.join(id: string)
```