// use kube::api;
use clap::Parser;
use kube::discovery;
use termtree::Tree;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short = 'A', long)]
    all: bool,
    #[arg(short, long)]
    namespace: Option<String>,
    #[arg(short, long)]
    global: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    // let mut tree = Tree::new("K8s Api Resource Tree");

    let client = kube::Client::try_default().await?;
    let d = cli.discovery(&client).await?;
    let t = cli.introspect(&d);
    println!("{t}");
    // traverse(&client, &mut tree).await?;
    // let tree = tree.build();
    // ptree::print_tree(&tree)?;
    Ok(())
}

impl Cli {
    async fn discovery(&self, client: &kube::Client) -> kube::Result<discovery::Discovery> {
        kube::Discovery::new(client.clone()).run().await
    }

    fn introspect(&self, discovery: &discovery::Discovery) -> Tree<String> {
        let mut tree = Tree::new("K8s Api Resource Tree".to_string());

        for ag in discovery.groups_alphabetical() {
            let group = api_group_to_tree(ag);
            tree.push(group);
        }

        tree
    }
}

// async fn traverse(client: &kube::Client, tree: &mut TreeBuilder) -> kube::Result<()> {
//     // let lp = api::ListParams::default();
//     let discovery = kube::Discovery::new(client.clone()).run().await?;
//     for group in discovery.groups_alphabetical() {
//         inspect_api_group(group, tree);
//     }
//     // discovery
//     //     .groups_alphabetical()
//     //     .into_iter()
//     //     .map(|group| (group, tree))
//     //     .for_each(inspect_api_group);

//     Ok(())
// }

fn api_group_to_tree(ag: &discovery::ApiGroup) -> Tree<String> {
    let group_name = match ag.name() {
        discovery::ApiGroup::CORE_GROUP => "core",
        other => other,
    };

    let mut tree = Tree::new(group_name.to_string());
    for (ar, _acap) in ag.recommended_resources() {
        tree.push(ar.plural);
    }

    tree
}

// fn inspect_api_group(group: &discovery::ApiGroup, tree: &mut TreeBuilder) {
//     let group_name = match group.name() {
//         discovery::ApiGroup::CORE_GROUP => "core",
//         other => other,
//     };

//     let group_name =
//         fmtools::fmt!({group_name} "/" {group.preferred_version_or_latest()}).to_string();

//     tree.begin_child(group_name);

//     for (ar, ac) in group.recommended_resources() {
//         inspect_resource(ar, ac, tree);
//     }

//     // group
//     //     .recommended_resources()
//     //     .into_iter()
//     //     .for_each(inspect_resource);

//     tree.end_child();
// }

// fn inspect_resource(
//     ar: discovery::ApiResource,
//     ac: discovery::ApiCapabilities,
//     tree: &mut TreeBuilder,
// ) {
//     let _namespaced = match ac.scope {
//         discovery::Scope::Cluster => "N",
//         discovery::Scope::Namespaced => "Y",
//     };
//     tree.add_empty_child(ar.plural);
//     // fmtools::fmt!({ar.plural:<32}{namespaced:>2});
// }
