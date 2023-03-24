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
            let value = {
                if *counter > 0 {
                    *counter - 1
                } else {
                    *counter
                }
            };
            counter.set(value);
        })
    };

    let reset = {
        let counter = counter.clone();
        Callback::from(move |_| {
            counter.set(0);
        })
    };

    html! {
        <div class={classes!("counter")}>
            <img src="img/WASM-logo.svg" alt="Logo of WebAssembly" width={ 200 }/>
            <h1>{ "Yew Counter" }</h1>
            <span class={classes!("counter__output")}>{*counter}</span>
            <div class={classes!("btn__container")}>
                <button class={classes!("control_btn")} onclick={increase}>{ "+1" }</button>
                <button class={classes!("control_btn")} onclick={decrease}>{ "-1" }</button>
                <button class={classes!("reset")} onclick={reset}>{ "Reset" }</button>
            </div>
        </div>
    }
}
