use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let counter = use_state(|| 0);

    let increase = {
        let counter = counter.clone();

        Callback::from(move |_| {
            let value = *counter + 1;
            counter.set(value);
        })
    };

    let decrease = {
        let counter = counter.clone();

        Callback::from(move |_| {
            let value = *counter - 1;
            counter.set(value);
        })
    };

    html! {
        <>
            <button onclick={increase}>{ "+1" }</button>
            <button onclick={decrease}>{ "-1" }</button>
            <p>{ *counter }</p>
        </>

    }
}
