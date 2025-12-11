use gloo_net::http::Request;
use web_sys::HtmlInputElement;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Item {
    pub id: i32,
    pub nome: String,
    pub preco: f64,
}

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize, Default)]
pub struct NovoItem {
    pub nome: String,
    pub preco: f64,
}

#[derive(Clone, PartialEq, serde::Deserialize, Default)]
pub struct ListarResponse {
    pub itens: Vec<Item>,
    pub total: i64,
    pub pagina: i64,
    pub por_pagina: i64,
    pub total_paginas: i64,
}

#[derive(Clone, PartialEq)]
enum View {
    Lista,
    Novo,
    Editar,
}

fn api_url() -> &'static str {
    "http://localhost:3000"
}

#[function_component(App)]
pub fn app() -> Html {
    let itens = use_state(|| Vec::<Item>::new());
    let carregando = use_state(|| false);
    let erro = use_state(|| None::<String>);

    let novo_nome = use_state(|| String::new());
    let novo_preco = use_state(|| String::new());
    let editar_id = use_state(|| String::new());

    let view = use_state(|| View::Lista);
    let nav_open = use_state(|| false);

    let busca = use_state(|| String::new());
    let busca_aplicada = use_state(|| String::new());
    let ordenar_por = use_state(|| "id".to_string());
    let ordem = use_state(|| "asc".to_string());
    let pagina = use_state(|| 1i64);
    let por_pagina = use_state(|| 10i64);
    let total = use_state(|| 0i64);
    let total_paginas = use_state(|| 0i64);

    let reload_trigger = use_state(|| 0u32);

    {
        let itens = itens.clone();
        let carregando = carregando.clone();
        let erro = erro.clone();
        let busca_aplicada = busca_aplicada.clone();
        let ordenar_por = ordenar_por.clone();
        let ordem = ordem.clone();
        let pagina = pagina.clone();
        let por_pagina = por_pagina.clone();
        let total = total.clone();
        let total_paginas = total_paginas.clone();

        let deps = (
            (*ordenar_por).clone(),
            (*ordem).clone(),
            *pagina,
            *por_pagina,
            (*busca_aplicada).clone(),
            *reload_trigger,
        );

        use_effect_with(deps, move |_| {
            let itens = itens.clone();
            let carregando = carregando.clone();
            let erro = erro.clone();
            let busca_val = (*busca_aplicada).clone();
            let ordenar_por_val = (*ordenar_por).clone();
            let ordem_val = (*ordem).clone();
            let pagina_val = *pagina;
            let por_pagina_val = *por_pagina;
            let total = total.clone();
            let total_paginas = total_paginas.clone();

            wasm_bindgen_futures::spawn_local(async move {
                carregando.set(true);
                erro.set(None);

                let mut url = format!(
                    "{}/itens?pagina={}&por_pagina={}&ordenar_por={}&ordem={}",
                    api_url(), pagina_val, por_pagina_val, ordenar_por_val, ordem_val
                );

                if !busca_val.trim().is_empty() {
                    url.push_str(&format!("&busca={}", busca_val));
                }

                let resp = Request::get(&url).send().await;

                match resp {
                    Ok(r) => match r.json::<ListarResponse>().await {
                        Ok(res) => {
                            itens.set(res.itens);
                            total.set(res.total);
                            total_paginas.set(res.total_paginas);
                        }
                        Err(e) => erro.set(Some(format!("Erro ao parsear resposta: {}", e))),
                    },
                    Err(e) => erro.set(Some(format!("Erro ao buscar itens: {}", e))),
                }

                carregando.set(false);
            });

            || ()
        });
    }

    let on_change_busca = {
        let busca = busca.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            busca.set(input.value());
        })
    };

    let on_change_por_pagina = {
        let por_pagina = por_pagina.clone();
        let pagina = pagina.clone();
        Callback::from(move |e: Event| {
            let select: HtmlSelectElement = e.target_unchecked_into();
            if let Ok(v) = select.value().parse() {
                por_pagina.set(v);
                pagina.set(1);
            }
        })
    };

    let on_buscar = {
        let busca = busca.clone();
        let busca_aplicada = busca_aplicada.clone();
        let pagina = pagina.clone();
        Callback::from(move |_| {
            busca_aplicada.set((*busca).clone());
            pagina.set(1);
        })
    };

    let on_change_nome = {
        let novo_nome = novo_nome.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            novo_nome.set(input.value());
        })
    };

    let on_change_preco = {
        let novo_preco = novo_preco.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            novo_preco.set(input.value());
        })
    };

    let incluir_item = {
        let novo_nome = novo_nome.clone();
        let novo_preco = novo_preco.clone();
        let erro = erro.clone();
        let carregando = carregando.clone();
        let view = view.clone();
        let reload_trigger = reload_trigger.clone();
        Callback::from(move |_| {
            let nome = (*novo_nome).clone();
            let preco_txt = (*novo_preco).clone();
            let erro = erro.clone();
            let carregando = carregando.clone();
            let view = view.clone();
            let reload_trigger = reload_trigger.clone();

            if nome.trim().is_empty() || preco_txt.trim().is_empty() {
                erro.set(Some("Nome e preço são obrigatórios".into()));
                return;
            }

            let preco: f64 = match preco_txt.parse() {
                Ok(v) => v,
                Err(_) => {
                    erro.set(Some("Preço inválido".into()));
                    return;
                }
            };

            wasm_bindgen_futures::spawn_local(async move {
                carregando.set(true);
                erro.set(None);

                let novo = NovoItem { nome, preco };

                let req = match Request::post(&format!("{}/itens", api_url()))
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&novo).unwrap())
                {
                    Ok(r) => r,
                    Err(e) => {
                        erro.set(Some(format!("Erro ao montar requisição: {}", e)));
                        carregando.set(false);
                        return;
                    }
                };

                let resp = req.send().await;

                match resp {
                    Ok(r) => match r.json::<Item>().await {
                        Ok(_) => {
                            view.set(View::Lista);
                            reload_trigger.set(*reload_trigger + 1);
                        }
                        Err(e) => erro.set(Some(format!("Erro ao criar item: {}", e))),
                    },
                    Err(e) => erro.set(Some(format!("Erro na requisição: {}", e))),
                }

                carregando.set(false);
            });
        })
    };

    let atualizar_item = {
        let editar_id = editar_id.clone();
        let novo_nome = novo_nome.clone();
        let novo_preco = novo_preco.clone();
        let erro = erro.clone();
        let carregando = carregando.clone();
        let view = view.clone();
        let reload_trigger = reload_trigger.clone();
        Callback::from(move |_| {
            let id_txt = (*editar_id).clone();
            let nome = (*novo_nome).clone();
            let preco_txt = (*novo_preco).clone();
            let erro = erro.clone();
            let carregando = carregando.clone();
            let view = view.clone();
            let reload_trigger = reload_trigger.clone();

            let id: i32 = match id_txt.parse() {
                Ok(v) => v,
                Err(_) => {
                    erro.set(Some("ID inválido".into()));
                    return;
                }
            };

            if nome.trim().is_empty() || preco_txt.trim().is_empty() {
                erro.set(Some("Nome e preço são obrigatórios para atualizar".into()));
                return;
            }

            let preco: f64 = match preco_txt.parse() {
                Ok(v) => v,
                Err(_) => {
                    erro.set(Some("Preço inválido".into()));
                    return;
                }
            };

            wasm_bindgen_futures::spawn_local(async move {
                carregando.set(true);
                erro.set(None);

                let atualizado = NovoItem { nome, preco };

                let req = match Request::put(&format!("{}/itens/{}", api_url(), id))
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&atualizado).unwrap())
                {
                    Ok(r) => r,
                    Err(e) => {
                        erro.set(Some(format!("Erro ao montar requisição: {}", e)));
                        carregando.set(false);
                        return;
                    }
                };

                let resp = req.send().await;

                match resp {
                    Ok(r) => match r.json::<Item>().await {
                        Ok(_) => {
                            view.set(View::Lista);
                            reload_trigger.set(*reload_trigger + 1);
                        }
                        Err(e) => erro.set(Some(format!("Erro ao atualizar item: {}", e))),
                    },
                    Err(e) => erro.set(Some(format!("Erro na requisição: {}", e))),
                }

                carregando.set(false);
            });
        })
    };

    let burger_class = if *nav_open { "navbar-burger is-active" } else { "navbar-burger" };
    let menu_class = if *nav_open { "navbar-menu is-active" } else { "navbar-menu" };

    let toggle_nav = {
        let nav_open = nav_open.clone();
        Callback::from(move |_| nav_open.set(!*nav_open))
    };

    let go_to_lista = {
        let view = view.clone();
        Callback::from(move |_| view.set(View::Lista))
    };

    let go_to_novo = {
        let view = view.clone();
        let editar_id = editar_id.clone();
        let novo_nome = novo_nome.clone();
        let novo_preco = novo_preco.clone();
        Callback::from(move |_| {
            editar_id.set(String::new());
            novo_nome.set(String::new());
            novo_preco.set(String::new());
            view.set(View::Novo);
        })
    };

    let cancelar = {
        let view = view.clone();
        Callback::from(move |_| view.set(View::Lista))
    };

    let is_edit = matches!(*view, View::Editar);

    let pagina_atual = *pagina;
    let total_pags = *total_paginas;
    let ordenar_por_atual = (*ordenar_por).clone();
    let ordem_atual = (*ordem).clone();

    let criar_ordenar_callback = |coluna: &'static str| {
        let ordenar_por = ordenar_por.clone();
        let ordem = ordem.clone();
        let pagina = pagina.clone();
        let ordenar_por_atual = ordenar_por_atual.clone();
        let ordem_atual = ordem_atual.clone();
        
        Callback::from(move |_| {
            if ordenar_por_atual == coluna {
                if ordem_atual == "asc" {
                    ordem.set("desc".to_string());
                } else {
                    ordem.set("asc".to_string());
                }
            } else {
                ordenar_por.set(coluna.to_string());
                ordem.set("asc".to_string());
            }
            pagina.set(1);
        })
    };

    let ordenar_id = criar_ordenar_callback("id");
    let ordenar_nome = criar_ordenar_callback("nome");
    let ordenar_preco = criar_ordenar_callback("preco");

    let seta = |coluna: &str| -> &'static str {
        if ordenar_por_atual == coluna {
            if ordem_atual == "asc" { " ▲" } else { " ▼" }
        } else {
            ""
        }
    };

    let seta_id = seta("id");
    let seta_nome = seta("nome");
    let seta_preco = seta("preco");

    let ir_para_pagina = |p: i64| {
        let pagina = pagina.clone();
        Callback::from(move |_| {
            pagina.set(p);
        })
    };

    let gerar_paginas = || -> Vec<i64> {
        let mut paginas = Vec::new();
        let total = total_pags;
        let atual = pagina_atual;
        
        if total <= 7 {
            for i in 1..=total {
                paginas.push(i);
            }
        } else {
            paginas.push(1);
            
            if atual > 3 {
                paginas.push(-1);
            }
            
            let inicio = (atual - 1).max(2);
            let fim = (atual + 1).min(total - 1);
            
            for i in inicio..=fim {
                if !paginas.contains(&i) {
                    paginas.push(i);
                }
            }
            
            if atual < total - 2 {
                paginas.push(-1);
            }
            
            if !paginas.contains(&total) {
                paginas.push(total);
            }
        }
        
        paginas
    };

    let paginas = gerar_paginas();

    html! {
        <>
            <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <a class="navbar-item">
                        <strong>{ "Rust Items" }</strong>
                    </a>
                    <a
                        role="button"
                        class={burger_class}
                        aria-label="menu"
                        aria-expanded={(*nav_open).to_string()}
                        onclick={toggle_nav}
                    >
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                    </a>
                </div>
                <div class={menu_class}>
                    <div class="navbar-start">
                        <a class="navbar-item" onclick={go_to_lista}>
                            { "Itens" }
                        </a>
                    </div>
                </div>
            </nav>

            if matches!(*view, View::Lista) {
                <section class="section">
                    <div class="container">
                        <div class="level">
                            <div class="level-left">
                                <div>
                                    <h1 class="title">{ "Itens" }</h1>
                                    <p class="subtitle">{ "Gerencie os itens consumindo a API Rust." }</p>
                                </div>
                            </div>
                            <div class="level-right">
                                <button class="button is-primary" onclick={go_to_novo.clone()}>{ "Incluir novo" }</button>
                            </div>
                        </div>

                        if *carregando {
                            <div class="notification is-info is-light">{ "Carregando..." }</div>
                        }

                        if let Some(msg) = &*erro {
                            <div class="notification is-danger is-light">{ msg }</div>
                        }

                        <div class="box">
                            <div class="columns is-vcentered">
                                <div class="column is-5">
                                    <div class="field has-addons">
                                        <div class="control is-expanded">
                                            <input
                                                class="input"
                                                type="text"
                                                placeholder="Buscar por ID ou nome..."
                                                value={(*busca).clone()}
                                                oninput={on_change_busca}
                                            />
                                        </div>
                                        <div class="control">
                                            <button class="button is-info" onclick={on_buscar}>{ "Buscar" }</button>
                                        </div>
                                    </div>
                                </div>
                                <div class="column is-3">
                                    <div class="field">
                                        <div class="control">
                                            <div class="select is-fullwidth">
                                                <select onchange={on_change_por_pagina}>
                                                    <option value="5" selected={*por_pagina == 5}>{ "5 por página" }</option>
                                                    <option value="10" selected={*por_pagina == 10}>{ "10 por página" }</option>
                                                    <option value="20" selected={*por_pagina == 20}>{ "20 por página" }</option>
                                                    <option value="50" selected={*por_pagina == 50}>{ "50 por página" }</option>
                                                </select>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                                <div class="column is-4 has-text-right">
                                    <span class="tag is-info is-medium">{ format!("{} itens", *total) }</span>
                                </div>
                            </div>
                        </div>

                        <div class="box">
                            if itens.is_empty() {
                                <p class="has-text-grey has-text-centered">{ "Nenhum item encontrado." }</p>
                            } else {
                                <div class="table-container">
                                    <table class="table is-fullwidth is-striped is-hoverable">
                                        <thead>
                                            <tr>
                                                <th class="is-clickable" onclick={ordenar_id} style="cursor: pointer;">
                                                    { format!("ID{}", seta_id) }
                                                </th>
                                                <th class="is-clickable" onclick={ordenar_nome} style="cursor: pointer;">
                                                    { format!("Nome{}", seta_nome) }
                                                </th>
                                                <th class="has-text-right is-clickable" onclick={ordenar_preco} style="cursor: pointer;">
                                                    { format!("Preço{}", seta_preco) }
                                                </th>
                                                <th class="has-text-centered">{ "Ações" }</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            { for itens.iter().map(|item| {
                                                let editar_id = editar_id.clone();
                                                let novo_nome = novo_nome.clone();
                                                let novo_preco = novo_preco.clone();
                                                let view = view.clone();
                                                let erro_del = erro.clone();
                                                let carregando_del = carregando.clone();
                                                let reload_trigger = reload_trigger.clone();

                                                let id = item.id;
                                                let nome = item.nome.clone();
                                                let preco = item.preco;

                                                let on_edit = Callback::from(move |_| {
                                                    editar_id.set(id.to_string());
                                                    novo_nome.set(nome.clone());
                                                    novo_preco.set(format!("{:.2}", preco));
                                                    view.set(View::Editar);
                                                });

                                                let on_delete = Callback::from(move |_| {
                                                    let erro = erro_del.clone();
                                                    let carregando = carregando_del.clone();
                                                    let reload_trigger = reload_trigger.clone();

                                                    wasm_bindgen_futures::spawn_local(async move {
                                                        carregando.set(true);
                                                        erro.set(None);

                                                        let resp = Request::delete(&format!("{}/itens/{}", api_url(), id))
                                                            .send()
                                                            .await;

                                                        match resp {
                                                            Ok(r) if r.status() == 204 => {
                                                                reload_trigger.set(*reload_trigger + 1);
                                                            }
                                                            Ok(r) => {
                                                                erro.set(Some(format!("Falha ao remover. Status: {}", r.status())));
                                                            }
                                                            Err(e) => erro.set(Some(format!("Erro na requisição: {}", e))),
                                                        }

                                                        carregando.set(false);
                                                    });
                                                });

                                                html! {
                                                    <tr>
                                                        <td>{ id }</td>
                                                        <td>{ &item.nome }</td>
                                                        <td class="has-text-right">{ format!("R$ {:.2}", item.preco) }</td>
                                                        <td class="has-text-centered">
                                                            <div class="buttons is-centered">
                                                                <button class="button is-small is-link is-light" onclick={on_edit}>{ "✏️" }</button>
                                                                <button class="button is-small is-danger is-light" onclick={on_delete}>{ "🗑️" }</button>
                                                            </div>
                                                        </td>
                                                    </tr>
                                                }
                                            })}
                                        </tbody>
                                    </table>
                                </div>

                                if total_pags > 1 {
                                    <nav class="pagination is-centered" role="navigation" aria-label="pagination">
                                        <a
                                            class={if pagina_atual <= 1 { "pagination-previous is-disabled" } else { "pagination-previous" }}
                                            onclick={ir_para_pagina((pagina_atual - 1).max(1))}
                                            disabled={pagina_atual <= 1}
                                        >
                                            { "<" }
                                        </a>
                                        <a
                                            class={if pagina_atual >= total_pags { "pagination-next is-disabled" } else { "pagination-next" }}
                                            onclick={ir_para_pagina((pagina_atual + 1).min(total_pags))}
                                            disabled={pagina_atual >= total_pags}
                                        >
                                            { ">" }
                                        </a>
                                        <ul class="pagination-list">
                                            { for paginas.iter().map(|&p| {
                                                if p == -1 {
                                                    html! {
                                                        <li>
                                                            <span class="pagination-ellipsis">{ "…" }</span>
                                                        </li>
                                                    }
                                                } else {
                                                    let is_current = p == pagina_atual;
                                                    let class = if is_current { "pagination-link is-current" } else { "pagination-link" };
                                                    html! {
                                                        <li>
                                                            <a class={class} onclick={ir_para_pagina(p)}>{ p }</a>
                                                        </li>
                                                    }
                                                }
                                            })}
                                        </ul>
                                    </nav>
                                }
                            }
                        </div>
                    </div>
                </section>
            }

            if matches!(*view, View::Novo | View::Editar) {
                <section class="section">
                    <div class="container">
                        <div class="box">
                            <h1 class="title is-4">
                                { if is_edit { "Editar item" } else { "Novo item" } }
                            </h1>
                            <p class="subtitle is-6">{ "Preencha os campos e salve." }</p>

                            if *carregando {
                                <div class="notification is-info is-light">{ "Carregando..." }</div>
                            }

                            if let Some(msg) = &*erro {
                                <div class="notification is-danger is-light">{ msg }</div>
                            }

                            if is_edit {
                                <div class="field">
                                    <label class="label">{ "ID" }</label>
                                    <div class="control">
                                        <input class="input" type="number" value={(*editar_id).clone()} disabled=true />
                                    </div>
                                </div>
                            }

                            <div class="field">
                                <label class="label">{ "Nome" }</label>
                                <div class="control">
                                    <input class="input" value={(*novo_nome).clone()} oninput={on_change_nome.clone()} placeholder="Nome do produto" />
                                </div>
                            </div>

                            <div class="field">
                                <label class="label">{ "Preço" }</label>
                                <div class="control">
                                    <input class="input" type="number" step="0.01" value={(*novo_preco).clone()} oninput={on_change_preco.clone()} placeholder="Ex: 49.90" />
                                </div>
                            </div>

                            <div class="buttons">
                                if is_edit {
                                    <button class="button is-link" onclick={atualizar_item.clone()}>{ "Salvar alterações" }</button>
                                } else {
                                    <button class="button is-primary" onclick={incluir_item.clone()}>{ "Salvar" }</button>
                                }
                                <button class="button" onclick={cancelar.clone()}>{ "Cancelar" }</button>
                            </div>
                        </div>
                    </div>
                </section>
            }
        </>
    }
}
