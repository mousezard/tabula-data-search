
pub mod Template {
    use std::sync::Arc;

    use handlebars::{Handlebars, JsonValue};
    use serde::Serialize;
    use serde_json::json;
    


    struct WithTemplate<T: Serialize> {
        name: &'static str,
        value: T,
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

    

    fn handlebar_init<T:Serialize>(template_name : &'static str ,template : String, content : T) -> impl warp::Reply{
        
        let mut hb = Handlebars::new();
        hb.register_template_string(&template_name,template).unwrap();

        let hb = Arc::new(hb);

        let it = move |with_template:WithTemplate<JsonValue>| render(with_template,hb.clone());

        it(WithTemplate{
            name :  { template_name },
            value : json!(content)
        })
    }
   
}
