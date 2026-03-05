use crate::{
    tab_component::TabComponent,
    util::{self, pretty_stockwerk_number},
};
use yew::{HtmlResult, component, html, suspense::use_future};

use crate::model::{Raum, Schrank};

#[component]
pub fn SidebarComponent() -> HtmlResult {
    let schrank_list = use_future(|| async {
        util::fetch::<Vec<Schrank>>("/api/schrank")
            .await
            .unwrap_or_default()
    })?;
    let raum_list = use_future(|| async {
        util::fetch::<Vec<Raum>>("/api/raum")
            .await
            .unwrap_or_default()
    })?;

    //let full_map = util::map_schraenke_to_ge_name(schrank_list.to_vec());
    let full_map = util::map_schraenke_raeume(schrank_list.to_vec(), raum_list.to_vec());

    let gebaeude_img = html! {
        <svg class="tab-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 640">
            <path style="fill: var(--color);" d="M192 112C183.2 112 176 119.2 176 128L176 512C176 520.8 183.2 528 192 528L272 528L272 448C272 430.3 286.3 416 304 416L336 416C353.7 416 368 430.3 368 448L368 528L448 528C456.8 528 464 520.8 464 512L464 128C464 119.2 456.8 112 448 112L192 112zM128 128C128 92.7 156.7 64 192 64L448 64C483.3 64 512 92.7 512 128L512 512C512 547.3 483.3 576 448 576L192 576C156.7 576 128 547.3 128 512L128 128zM224 176C224 167.2 231.2 160 240 160L272 160C280.8 160 288 167.2 288 176L288 208C288 216.8 280.8 224 272 224L240 224C231.2 224 224 216.8 224 208L224 176zM368 160L400 160C408.8 160 416 167.2 416 176L416 208C416 216.8 408.8 224 400 224L368 224C359.2 224 352 216.8 352 208L352 176C352 167.2 359.2 160 368 160zM224 304C224 295.2 231.2 288 240 288L272 288C280.8 288 288 295.2 288 304L288 336C288 344.8 280.8 352 272 352L240 352C231.2 352 224 344.8 224 336L224 304zM368 288L400 288C408.8 288 416 295.2 416 304L416 336C416 344.8 408.8 352 400 352L368 352C359.2 352 352 344.8 352 336L352 304C352 295.2 359.2 288 368 288z"/>
        </svg>
    };
    let stockwerk_img = html! {
        <svg class="tab-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 640">
            <path style="fill: var(--color);" d="M296.5 69.2C311.4 62.3 328.6 62.3 343.5 69.2L562.1 170.2C570.6 174.1 576 182.6 576 192C576 201.4 570.6 209.9 562.1 213.8L343.5 314.8C328.6 321.7 311.4 321.7 296.5 314.8L77.9 213.8C69.4 209.8 64 201.3 64 192C64 182.7 69.4 174.1 77.9 170.2L296.5 69.2zM112.1 282.4L276.4 358.3C304.1 371.1 336 371.1 363.7 358.3L528 282.4L562.1 298.2C570.6 302.1 576 310.6 576 320C576 329.4 570.6 337.9 562.1 341.8L343.5 442.8C328.6 449.7 311.4 449.7 296.5 442.8L77.9 341.8C69.4 337.8 64 329.3 64 320C64 310.7 69.4 302.1 77.9 298.2L112 282.4zM77.9 426.2L112 410.4L276.3 486.3C304 499.1 335.9 499.1 363.6 486.3L527.9 410.4L562 426.2C570.5 430.1 575.9 438.6 575.9 448C575.9 457.4 570.5 465.9 562 469.8L343.4 570.8C328.5 577.7 311.3 577.7 296.4 570.8L77.9 469.8C69.4 465.8 64 457.3 64 448C64 438.7 69.4 430.1 77.9 426.2z"/>
        </svg>
    };
    let divider = html! {  <div class="divider" /> };

    Ok(html! {
        <div id="sidebar">
            <div id="sidebarTitle">{"Locations"}</div>
            <div id="sidebarContent">
                for (ge_name, stockwerk_map) in full_map {
                    <TabComponent title={ge_name.clone()} img={gebaeude_img.clone()}>
                        for (stockwerk, (schraenke, raeume)) in stockwerk_map {
                            <TabComponent title={pretty_stockwerk_number(stockwerk)} img={stockwerk_img.clone()}>
                                <div class="schraenke">

                                    for schrank in schraenke.iter() {
                                        <div class="schrank">
                                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 640">
                                                <path style="fill: var(--color);" d="M465.4 192L431.1 144L209 144L174.7 192L465.4 192zM96 212.5C96 199.2 100.2 186.2 107.9 175.3L156.9 106.8C168.9 90 188.3 80 208.9 80L431 80C451.7 80 471.1 90 483.1 106.8L532 175.3C539.8 186.2 543.9 199.2 543.9 212.5L544 480C544 515.3 515.3 544 480 544L160 544C124.7 544 96 515.3 96 480L96 212.5z"/>
                                            </svg>
                                            <div>
                                                {schrank.sc_nummer.clone()}
                                            </div>
                                        </div>
                                    }
                                </div>
                                {
                                    if !schraenke.is_empty() && !raeume.is_empty() {
                                        divider.clone()
                                    } else {
                                        html! {""}
                                    }
                                }
                                <div class="raeume">
                                    for raum in raeume {
                                        <div class="raum">
                                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 640">
                                                <path style="fill: var(--color);" d="M128 128C128 92.7 156.7 64 192 64L448 64C483.3 64 512 92.7 512 128L512 512C529.7 512 544 526.3 544 544C544 561.7 529.7 576 512 576L128 576C110.3 576 96 561.7 96 544C96 526.3 110.3 512 128 512L128 128zM416 352C433.7 352 448 337.7 448 320C448 302.3 433.7 288 416 288C398.3 288 384 302.3 384 320C384 337.7 398.3 352 416 352z"/>
                                            </svg>
                                            <div>
                                                {raum.ra_nummer}
                                            </div>
                                        </div>
                                    }
                                </div>
                            </TabComponent>
                        }
                    </TabComponent>
                }
            </div>
        </div>
    })
}
/*
for (ge_name, stockwerk_map) in full_map {
    <div class="tab" key={ge_name.clone()}>
        <div class="tab-title">
            <svg class="arrow" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 640">
                <path style="fill: var(--color);" d="M297.4 438.6C309.9 451.1 330.2 451.1 342.7 438.6L502.7 278.6C515.2 266.1 515.2 245.8 502.7 233.3C490.2 220.8 469.9 220.8 457.4 233.3L320 370.7L182.6 233.4C170.1 220.9 149.8 220.9 137.3 233.4C124.8 245.9 124.8 266.2 137.3 278.7L297.3 438.7z"/>
            </svg>
            <div>
                {ge_name}
            </div>
        </div>
        <div class="tab-content">
            for (stockwerk, schraenke) in stockwerk_map {
                <div class="tab">
                    <div class="tab-title">

                    </div>
                    <div class="tab-content">

                    </div>
                </div>
                <div class="stockwerk-title">{format!("Stockwerk: {}", stockwerk)}</div>
                <div class="schraenke-container">
                    for schrank in schraenke {
                        <div class="schrank">
                            {format!("Schrank: {}", schrank.sc_nummer)}
                        </div>
                    }
                </div>
            }
        </div>
    </div>
}*/
