mod components;
mod data;
mod router;
mod translations;

use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Router};

use components::{book::Book, books::Books, breadcrumb::BreadCrumb, footer::Footer, title::Title};
use data::BOOKS;
use router::AppRoutes;
use translations::get_message_global;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    sycamore::render(|ctx| {
        let lang = ctx.create_signal("cs".to_owned());
        view! { ctx,
            Router {
                integration: HistoryIntegration::new(),
                view: move |ctx, route: &ReadSignal<AppRoutes>| {
                    let levels = ctx.create_signal(vec![] as Vec<String>);
                    let not_found_text = get_message_global("not_found", &lang.get(), None);
                    view! { ctx,
                        div(class="root") {
                            div(class="section page-header pb-1 pt-0") {
                                Title {}
                                BreadCrumb {
                                    levels: levels,
                                }
                            }
                            main(class="section is-flex") {
                                (
                                    match route.get().as_ref() {
                                        AppRoutes::NotFound => view! { ctx,
                                            div(class="box notification is-warning") {
                                                span(class="icon") {
                                                    i(class="fas fa-exclamation"){}
                                                }
                                                strong{(not_found_text)}
                                            }
                                        },
                                        AppRoutes::Root => {
                                            levels.set(vec![]);
                                            view! { ctx,
                                                Books{
                                                    lang: lang,
                                                    levels: levels,
                                                }
                                            }
                                        },
                                        AppRoutes::Book {book_slug} => {
                                            let book_slug = book_slug.to_owned();

                                            let matches = BOOKS.iter().filter(|b| b.name == book_slug[0]).collect::<Vec<_>>();
                                            levels.set(book_slug);

                                            if !matches.is_empty() {

                                                view! { ctx,
                                                    Book {
                                                        lang: lang,
                                                        book: matches[0].clone(),
                                                    }
                                                }
                                            } else {
                                                view! { ctx,
                                                    div(class="box notification is-warning w-100") {
                                                        span(class="icon") {
                                                            i(class="fas fa-exclamation"){}
                                                        }
                                                        strong{(not_found_text)}
                                                    }
                                                }
                                            }
                                        },
                                    }
                                )
                            }
                            Footer {}
                        }
                    }
                },

            }
        }
    });
}