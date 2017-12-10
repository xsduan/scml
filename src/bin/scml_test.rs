extern crate scml;

fn main() {
        let scml_str = r#"
{
  "strokes":[
    {
      "type":"s",
      "anchors":[
        {
          "id":0,
          "at":"begin"
        },
        {
          "id":1,
          "at":"inside"
        },
        {
          "id":2,
          "at":"end"
        }
      ],
      "locations":[

      ]
    },
    {
      "type":"hz",
      "anchors":[
        {
          "id":0,
          "at":"begin"
        },
        {
          "id":3,
          "at":"inside2"
        },
        {
          "id":4,
          "at":"end"
        }
      ],
      "locations":[

      ]
    },
    {
      "type":"h",
      "anchors":[
        {
          "id":1,
          "at":"begin"
        },
        {
          "id":3,
          "at":"end"
        }
      ],
      "locations":[

      ]
    },
    {
      "type":"h",
      "anchors":[
        {
          "id":2,
          "at":"begin"
        },
        {
          "id":4,
          "at":"end"
        }
      ],
      "locations":[

      ]
    }
  ]
}
    "#;

    println!("{:#?}", scml::parse::read(&scml_str));
}
