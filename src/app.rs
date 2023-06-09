use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        // <Link rel="copy-file" type_="image/ico" href="/favicon.ico"/>
        // <link data-trunk rel="copy-file" type="image/svg" href="/public/logo.svg"/>
        // <link data-trunk rel="copy-file" type="image/png" href="public/logo.png"/>
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route path="" view=  move |cx| view! { cx, <Home/> }/>
            </Routes>
        </Router>
    }
}

#[component]
fn Header(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <section>
            <div class="bg-black relative text-white">
            <nav class="container mx-auto relative">
                <a href="#" class="font-bold font-serif hover:text-opacity-75 inline-flex items-center leading-none mr-4 text-white text-xl uppercase">     
                // <Link rel="copy-file" type_="image/ico" href="/public/favicon.ico"/>
                // <link data-trunk rel="copy-file" type="image/svg" href="/public/logo.svg"/>
                    <img class="m-6" src="/public/logo-white.svg" alt="Logo da Reality Check -- duas metades desalinhadas de um espiral com uma linha as dividindo" width="30" height="30"/>
                    <span><p class="font-sans">"Reality check"</p></span> 
                </a>
            </nav>
            </div>
        </section>
    }
}

#[component]
fn Hero(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <section class="bg-white dark:bg-gray-900">
           <div class="mx-auto w-full text-center flex place-items-center" style="background-image: url('/public/hero.jpg');height:480px;">
                // <img class="w-full" src="/public/hero.jpg" style="height:80%;" />
                <div class="">
                    <p class="object-center align-middle text-white font-sans text-9xl font-bold tracking-wide">"REALITY CHECK"</p>
                </div>
            </div>
        </section>
    }
}

#[component]
fn QuemSomos(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <section class="p-8 bg-violet-900">
            <div>
                <h2 class="text-center text-slate-100 text-2xl font-sans my-4">"Quem somos?"</h2>
                <div class="mx-16">
                    <p class="mb-4 text-slate-200">"A reality check é uma empresa que oferece soluções em análise de dados e análises quantitativas para ciências humanas e sociais. A empresa foi fundada por Vini Lemos, que é formado e mestre em psicologia."</p>
                    <p class="mb-4 text-slate-200">"Na reality check acreditamos que a análise de dados pode ajudar pesquisadores e empresas a serem mais assertivos e confiantes em suas análises, identificando padrões, tendências, correlações e causalidades nos fenômenos sociais."</p>
                    <p class="mb-4 text-slate-200">"Nós utilizamos ferramentas como o R e Python para realizar análises de dados de forma transparente, simples, reprodutível e compreensível."</p>
                </div>
            </div>
        </section>
    }
}

#[component]
fn QueFazemos(cx: Scope, 
    titulo: &'static str, 
    texto: &'static str, 
    class: &'static str,
    image_path:&'static str) -> impl IntoView {
    view!{
        cx,
            <div class={class}>
                <div class="flex flex-row my-4">
                    <div class="mx-auto w-96 text-center">
                        <img class="mx-auto rounded-full border-white border-4 w-36" src={image_path}/>
                        <p class="font-bold pt-2">{titulo}</p>
                        <p>{texto}</p>
                        // <p>"Análise quantitativa"</p>
                        // <p>"Use métodos matemáticos e estatísticos para transformar informação em conhecimento"</p>
                    </div>
                </div>
            </div>
    }
}

#[component]
fn Contato(cx: Scope) -> impl IntoView {
    view!{cx,
        <section class="p-8 bg-violet-500">
        <h2 class="text-center text-slate-100 text-2xl font-sans my-4">"Entre em contato"</h2>
            <div class="flex justify-center">
                <div class="text-center text-slate-200">
                    <p>"Estamos aqui para ajudar :)"</p>
                    <p>"contato@realitycheck.com.br"</p>
                    <p>"Vinicius Lemos (Whatsapp)"</p>
                    <p>"21 98065-3600"</p>
                    <button class="bg-teal-400 w-48 h-10 rounded-full border-4 border-teal-200">
                        <a class="text-slate-900 text-bold" href="https://wa.me/5521980653600">"Iniciar Conversa"</a>
                    </button>
                    <img class="mx-auto pt-8 drop-shadow-md" src="/public/rlt/tam15_t.png" alt="Aquarela de um Tamanduá estilizado feito com IA gerativa" width="120" height="120"/>
                </div>
            </div>
        </section>
    }
}

#[component]
fn Footer(cx:Scope) -> impl IntoView {
    view!{cx,
        <section class="bg-slate-900 h-96">
            <div class="flex justify-center">
                <img class="pt-20" src="/public/logo-white.svg" alt="Logo da Reality Check -- duas metades desalinhadas de um espiral com uma linha as dividindo" width="80" height="80"/>
            </div>
        </section>
    }
}

#[component]
fn Home(cx: Scope) -> impl IntoView {
    // let (count, set_count) = create_signal(cx, 0);
    view! { cx,
        <Header/>
        <Hero/>
        <section class="bg-violet-800 text-slate-200 py-8">
            <h2 class="text-center text-2xl py-4 font-sans text-slate-100">"O que podemos fazer por você?"</h2>
            <div class="flex flex-wrap mx-auto justify-center">
                <QueFazemos image_path="/public/rlt/verde6.jpg" class="basis-1/4" titulo="Análise Quantitativa" texto="Use métodos matemáticos e estatísticos para transformar informação em conhecimento"/>
                <QueFazemos image_path="/public/rlt/verde1.jpg" class="basis-1/4" titulo="Processamento de Linguagem Natural" texto="Entenda o que está por trás de dados escritos e falados"/>
                <QueFazemos image_path="/public/rlt/verde2.jpg" class="basis-1/4" titulo="Planejamento de pesquisa" texto="Nós podemos te ajudar a entender os passos necessários para aumentar a chance de sucesso do seu projeto ou produto"/>
                <QueFazemos image_path="/public/rlt/verde4.jpg" class="basis-1/4" titulo="Análise preditiva" texto="Use métodos matemáticos e estatísticos para transformar informação em conhecimentoCrie modelos para projetar o que irá acontecer no futuro e diminuir a sua incerteza"/>
            </div>
        </section>
        // <QueFazemos/>
        <QuemSomos/>
        <Contato/>
        <Footer/>
        // <div class="my-0 mx-auto max-w-3xl text-center">
        //     <h2 class="p-6 text-4xl">"Welcome to Leptos with Tailwind"</h2>
        //     <p class="px-10 pb-10 text-left">"Tailwind will scan your Rust files for Tailwind class names and compile them into a CSS file."</p>
        //     <button
        //         class="bg-amber-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg"
        //         on:click=move |_| set_count.update(|count| *count += 1)
        //     >
        //         "Something's here | "
        //         {move || if count() == 0 {
        //             "Click me!".to_string()
        //         } else {
        //             count().to_string()
        //         }}
        //         " | Some more text"
        //     </button>
        // </div>
    }
}
