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

you can also use .md files as components as if they where normal html components

