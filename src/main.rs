// use kube::api;
use kube::discovery;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = kube::Client::try_default().await?;
    traverse(&client).await?;
    Ok(())
}

async fn traverse(client: &kube::Client) -> kube::Result<()> {
    // let lp = api::ListParams::default();
    let discovery = kube::Discovery::new(client.clone()).run().await?;
    discovery
        .groups_alphabetical()
        .into_iter()
        .for_each(inspect_api_group);

    Ok(())
}

fn inspect_api_group(group: &discovery::ApiGroup) {
    let group_name = match group.name() {
        discovery::ApiGroup::CORE_GROUP => "core",
        other => other,
    };
    fmtools::println!({group_name} "/" {group.preferred_version_or_latest()});
    group
        .recommended_resources()
        .into_iter()
        .for_each(inspect_resource);
}

fn inspect_resource((ar, ac): (discovery::ApiResource, discovery::ApiCapabilities)) {
    let namespaced = match ac.scope {
        discovery::Scope::Cluster => "N",
        discovery::Scope::Namespaced => "Y",
    };
    fmtools::println!("  "{ar.plural:<32}{namespaced:>2});
}
