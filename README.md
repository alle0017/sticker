# sticker 🦙
Simple html compiler for component syntax. No shadow dom generation nor particular type of rendering. Simple static html generation.

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
cargo run mypage.html mypagecompiled.html
```

or

```bash
cargo run
```

enter the path of the file you want to compile and the filename of the result

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
 you can also import sticker.js in your project to use dynamic components creation. you have only to specify in the sticker tag that, the specific component, needs to be dynamic. then, you can use the following functions to create and modify components
 ```html
 <div>
  hello {{name}}
</div>
 ```

 ```html
<sticker>
  #use hello.html as hello dynamic;
</sticker>
 ```

 ```javascript
 import { Sticker } from './sticker.js';
let component = Sticker.append('hello');
component.setAttribute('name', 'world');
 ```
the function implemented are:
 ```typescript  
 /**
 * create custom component and append it. node is document.body by default
 */
Sticker.append(name: string, node: HTMLElement): HTMLElement;
/**
* set the attribute name of the component with the attribute value
*/
component.setAttribute(name: string, value: string): void;
/**
* if condition is true, append custom element
*/
Sticker.if( name: string, condition: boolean, node: HTMLElement): void;
/**
* if condition is true, append custom first specified custom element, else append second custom element
*/
Sticker.ifElse( ifName: string, elseName: string, condition: boolean, node: HTMLElement ): void;
/**
* create elements of type name, and assign to each one the attribute *specified in each attribute.
*/
Sticker.for(name: string,attributes: Attribute[]): (node: HTMLElement)=>void;

//where first string is attribute name, the second is the attribute value
type Attribute = Record<string,string>;
 ```

