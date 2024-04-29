use atspi::events::object::ChildrenChangedEvent;
use atspi::{
    connection::set_session_accessibility, events::object::ObjectEvents,
    proxy::accessible::ObjectRefExt, AccessibilityConnection,
};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    set_session_accessibility(true).await?;
    let conn = AccessibilityConnection::new().await?;

    conn.register_event::<ObjectEvents>().await?;

    let events = conn.event_stream();
    tokio::pin!(events);

    while let Some(Ok(ev)) = events.next().await {
        let Ok(chicha) = <ChildrenChangedEvent>::try_from(ev.clone()) else {
            continue;
        };

        println!("Operation: {}", chicha.operation);

        let object = chicha.item;
        let Ok(proxy) = object.as_accessible_proxy(conn.connection()).await else {
            eprintln!(
                "Failed to obtain an `AccessobleProxy` for object: {:#?}",
                object
            );
            continue;
        };

        let Ok(name) = proxy.name().await else {
            eprintln!("Failed to obtain the name of the object: {:#?}", object);
            continue;
        };
        println!("Server's name: {}", &name);

        let children = proxy.get_children().await?;
        for child in children {
            let child_proxy = child.as_accessible_proxy(conn.connection()).await?;
            let name = child_proxy.name().await?;
            let role = child_proxy.get_role().await?;
            let description = child_proxy.description().await?;
            println!("Name: {name}, Role: {role}, Description: {description}");
        }
    }

    Ok(())
}
