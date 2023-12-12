import { Sticker } from './sticker.js';

function main(){
      const button = document.getElementById('submit');
      if( !button )
            return;
      button.addEventListener('click', ()=>{
            const textBox = document.getElementById('new-todo');
            if( !textBox ) 
                  return;
            const elem = Sticker.appendCustomElement( 'todo', document.getElementById('todo-list') );
            elem.setAttribute('todo', textBox.value);
            textBox.value = '';
      })     
}
main();