export class MdRouter {
      #routes = {}
      #md;
      /**
      * @constructor
      * @param {<{ name: string, link: string }>Array} routes use it to configure the routes to md files
       */
      constructor(routes){
            if( routes instanceof Array ){
                  for( let route of routes ){
                        if( this.#isRoute( route ) ){
                              this.#routes[ route.name ] = route.link;
                        }
                  }
            }
            this.#md = document.getElementById('md-app');
            if( !this.#md ){
                  throw 'no md-app found. ensure to add to your html file the "mdApp.html" component';
            }
            this.#md.src = routes[0].link;
      }
      /**
       * 
       * @param {string} route 
       * go to a configured route
       */
      goto(route){
            if(  typeof route == 'string' && route in this.#routes )
                  this.#md.src = this.#routes[route];
            else if( typeof route != 'string' ){
                  console.warn(`route ${route} is not typeof string`);
            }else {
                  console.warn(`route ${route} does not exist `);
                  console.table(this.#routes);
            }
      }
      #isRoute( route ){
            return (
            'name' in route &&
            'link' in route &&
            typeof route.link == 'string' &&
            typeof route.name == 'string' );
      }
}
