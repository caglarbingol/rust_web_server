use actix_web::{HttpResponse, Responder};

pub async fn home() -> impl Responder {
    let dynamic_content = format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Dynamic HTML Example</title>
            <style>
                body {{
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    height: 100vh;
                    margin: 0;
                    background-color: #000;
                }}
                #dynamic-variable {{
                    font-size: 3em;
                    color: #fff;
                    background-color: #333;
                    border-radius: 15px;
                    padding: 20px;
                }}
            </style>
            <script>
                const eventSource = new EventSource('/events');
                eventSource.onmessage = (event) => {{
                    document.getElementById('dynamic-variable').textContent = event.data;
                }};
            </script>
        </head>
        <body>
            <span id="dynamic-variable"></span>
        </body>
        </html>
        "#);

    HttpResponse::Ok()
        .content_type("text/html")
        .body(dynamic_content)
}