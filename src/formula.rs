use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = katex, js_name = renderToString)]
    fn katex_render(latex: &str, opts: &JsValue) -> String;
}

fn render_latex(latex: &str) -> String {
    let opts = js_sys::Object::new();
    js_sys::Reflect::set(
        &opts,
        &JsValue::from_str("displayMode"),
        &JsValue::from_bool(true),
    ).unwrap();
    katex_render(latex, &opts)
}

#[component]
pub fn KnapsackFormula() -> impl IntoView {
    let latex = r#"
        dp[i][w] = \begin{cases}
            0 & \text{if } i = 0 \text{ or } w = 0 \\[6pt]
            dp[i-1][w] & \text{if } wt_i > w \\[6pt]
            \max\bigl(dp[i-1][w],\ dp[i-1][w - wt_i] + b_i\bigr) & \text{if } wt_i \leq w
        \end{cases}
    "#;

    let html = render_latex(latex);

    view! {
        <section class="formula-card">
            <h2 class="formula-title">"Recurrent Function"</h2>
            <div class="formula-body" inner_html=html />
            <div class="formula-legend">
                <span><strong class="accent">"i"</strong>" — item index"</span>
                <span><strong class="accent">"w"</strong>" — current capacity"</span>
                <span><strong class="accent">"wt"</strong><sub>"i"</sub>" — weight of item i"</span>
                <span><strong class="accent">"b"</strong><sub>"i"</sub>" — benefit of item i"</span>
            </div>
        </section>
    }
}
