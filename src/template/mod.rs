pub mod Template {
    use std::sync::Arc;

    use handlebars::{Handlebars, JsonValue};
    use serde::Serialize;
    use serde_json::json;


    pub const TEMPLATE: &str =  
    "<!DOCTYPE html>
    <html>
      <head>
      <link rel=\"stylesheet\" href=\"https://cdn.jsdelivr.net/npm/water.css@2/out/water.css\">
        <title>Search Result</title>
      </head>
      <body>
      <form action=\"/search\" >
      <div class=\"row\">
      <label for=\"search\">search</label>
      <input id=\"search\" name=\"search\" type=\"search\" placeholder=\"search here\"> 
      <input type=\"submit\">
      </div> 
      </form>
        <h1>Result of Search \"{{query}}\"!</h1>
        <table>
        <thead>
        <th>File name </th>
        <th>Content</th>
        </thead>
        <tbody>
        
        {{#each res}}
        <tr>
        <td>{{this.0}}</td>
        <td>{{this.1}}</td>
        </tr>
    {{/each}}
    </tbody>

        </table>
        <ol>
        <ol>
      </body>
    </html>";

    pub const SEARCH_TEMPLATE: &str = "<!DOCTYPE html>
    <html>
      <head>
      <link rel=\"stylesheet\" href=\"https://cdn.jsdelivr.net/npm/water.css@2/out/water.css\">
        <title>Tabula Search</title>
      </head>
      <body>
        <form action=\"/search\">
        <label for=\"search\">search</label>
        <input id=\"search\" name=\"search\" type=\"search\" placeholder=\"search here\"> 
        <input type=\"submit\"> 
        </form>
      </body>
    </html>";    

    #[derive(Serialize)]
    pub struct WithTemplate<T: Serialize> {
        pub(crate) name: &'static str,
        pub(crate) value: T,
    }

    fn render<T>(template: WithTemplate<T>, hbs: Arc<Handlebars<'_>>) -> impl warp::Reply
    where
        T: Serialize,
    {
        let render = hbs
            .render(template.name, &template.value)
            .unwrap_or_else(|err| err.to_string());
        warp::reply::html(render)
    }
    /*
        Handlebar changed to shared instances so we can cobine it
    */

    pub fn handlebar<T: Serialize>(
        template_name: &'static str,
        content: T,
        template: String,
    ) -> impl warp::Reply {
        let mut hb = Handlebars::new();
        hb.register_template_string(&template_name, template)
            .unwrap();

        let hb = Arc::new(hb);

        let it = move |with_template: WithTemplate<T>| render(with_template, hb.clone());

        it(WithTemplate {
            name: { template_name },
            value: content,
        })
    }
}
