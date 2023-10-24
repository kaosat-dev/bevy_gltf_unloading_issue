use bevy::{gltf::Gltf, prelude::*};


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                ..default()
            }),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, track_creation)
        .add_systems(Update, track_loadstate)
        .run();
}


#[derive(Resource)]
pub struct MyGltf ( pub Handle<Gltf> );

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // does not workcommands.insert_resource(MyGltf(asset_server.load("missiletower_building002.glb#Scene0")));
    //commands.insert_resource(MyGltf(asset_server.load("Wheelbot.glb#Scene0")));
    //commands.insert_resource(MyGltf(asset_server.load("free_gmc_motorhome_reimagined_low_poly_high.glb#Scene0")));

    //commands.insert_resource(MyGltf(asset_server.load("FlightHelmet.gltf#Scene0")));
    commands.insert_resource(MyGltf(asset_server.load("missiletower1_building003.glb#Scene0")));

    
}

fn track_creation(
    mut events: EventReader<AssetEvent<Gltf>>,
) {
    for event in events.iter() {
        if let AssetEvent::Created { handle } = event {
            info!("gltf created {:?}", handle.clone());
        }
    }
}

fn track_loadstate(
    asset_server: Res<AssetServer>,
    bla: ResMut<MyGltf>
){
    info!(
        "checking for loaded gltfs {:?}",
        asset_server.get_load_state(&bla.0)
    );
}