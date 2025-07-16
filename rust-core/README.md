The app's logic is dependent on cargo to install crates.
To get it up and running type
<pre>cargo run</pre>
Currently only creating items via the media api has been implemented.
A sample call with some dummy data would be
<pre>$ curl -X POST http://localhost:3000/api/media   -H "Content-Type: application/json"   -d '{                                                                                        
    "sha1": [218, 57, 163, 238, 94, 107, 75, 13, 50, 85, 191, 239, 149, 96, 24, 144, 175, 216, 7, 9],     
    "remote_id": "some_remote_id_123",
    "local_reference": "/home/Pictures/image2.jpg",                          
    "local_size": 123456,
    "key": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
    "meta_data": [10, 20, 30, 40, 50, 60, 70, 80],
    "type": 1,                                    
    "state": 0,                                   
    "file_name": "MyAwesomeVacationPic.jpg",
    "sort_order": 0,
    "error_count": 0,
    "version": 1,
    "format": 1,
    "error_message": "",
    "local_bucket": "default_bucket",
    "liked": 0,                      
    "hidden": 0,                      
    "dirty": 0
  }'
</pre>
