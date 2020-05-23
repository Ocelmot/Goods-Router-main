use db_connector::GRConnection;
use db_connector::location::Location;
use db_connector::geo_types::{Point, Polygon};

use db_connector::ObjectId;

use db_connector::bfs::BFS;

use serde_json;
use bson;

fn main() {
    let connector = GRConnection::new("mongodb://localhost:27017").unwrap();
    let bfs = BFS::new(connector);
    let connector2 = GRConnection::new("mongodb://localhost:27017").unwrap();

    // let loc = Location::new(Point{(-73.856077, 40.848447)}, Polygon::Radius(5.0));

    let search_area = Polygon{coords: vec![vec![

        ( -73.85249385217892, 40.85268770549687 ), ( -73.861159720278, 40.85077222651262 ), ( -73.86236092971747, 40.84515533330003 ), ( -73.85412406498965, 40.84096670421789 ), ( -73.84708840970131, 40.84707097465675 ), ( -73.85249385217892, 40.85268770549687 ) 

    ]]};

    let domain = Polygon{coords:vec![vec![( -73.83787766329989, 40.861903651984925 ), ( -73.8792335882875, 40.84969728171298 ), ( -73.86739309524125, 40.83099379974825 ), ( -73.82603717025364, 40.83631963410365 ), ( -73.83787766329989, 40.861903651984925 ) ]]};


    //start: 5ec45328009f5cc500f8a55a
    let start_id = ObjectId::with_string("5ec45328009f5cc500f8a55a").unwrap();
    let resource = String::from("test");

    let start_location = connector2.get_location(start_id).unwrap();

    let path = bfs.bfs(start_location, resource);

    println!("path: {:?}", path);
    // let mut loc = Location::new(Point{coords:(-73.87619018554688,40.839398489410435)}, domain.clone());
    // connector.save_location(&mut loc).unwrap();


    // let mut loc = Location::new(Point{coords:(-73.8559341430664,40.84693050130356)}, domain.clone());
    // connector.save_location(&mut loc).unwrap();


    // let mut loc = Location::new(Point{coords:(-73.85610580444336,40.84433335251483)}, domain.clone());
    // connector.save_location(&mut loc).unwrap();


    // let mut loc = Location::new(Point{coords:(-73.85421752929688,40.86108317321337)}, domain.clone());
    // connector.save_location(&mut loc).unwrap();
    // let serialized = serde_json::to_string(&loc).unwrap();
    // println!("serialized = {}", serialized);
    // let deserialized: Location = serde_json::from_str(&serialized).unwrap();
    // print!("deserialized: {:?}", deserialized);


    // println!("Before {:?}", loc);
    
    // println!("After {:?}", loc);


    


    // let locations = connector.get_locations().unwrap();
    // // let locations = connector.get_locations_within(search_area).unwrap();
    // for location in locations{
    //     let location = location.unwrap();
    //     let location :Location= bson::from_bson(bson::Bson::Document(location)).unwrap();


    //     let serialized = serde_json::to_string(&location).unwrap();
    //     println!("serialized = {}", serialized);

    // }
}

