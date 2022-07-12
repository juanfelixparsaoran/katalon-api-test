Nama : Juan Felix Parsaoran Tarigan <br>
Kode Peserta : KSAT006ONL017

# katalon-api-test

## Preview
This repo is a katalon project for testing http://jsonplaceholder.typicode.com/. There are 6 endpoints and this project will test method GET,POST, and DELETE from each endpoints

## Test Scenario
### /posts
- GET all posts and check all the response body with looping and status code (200 ok)
- GET post by one id and check the response body and status code (200 ok)
- GET posts by 10 id with looping and check the response body and status code (200 ok)
- POST post with request body and check the response and status code (201 created)
- DELETE post with id and check the response body and status code (204 no content)

### /comments
- GET all comments and check all the response body with looping and status code (200 ok)
- GET comments by one id and check the response body and status code (200 ok)
- GET comments by 10 id with looping and check the response body and status code (200 ok)
- POST comments with request body and check the response and status code (201 created)
- DELETE comments with id and check the response body and status code (204 no content)

### /albums
- GET all albums and check all the response body with looping and status code (200 ok)
- GET albums by one id and check the response body and status code (200 ok)
- GET albums by 10 id with looping and check the response body and status code (200 ok)
- POST albums with request body and check the response and status code (201 created)
- DELETE albums with id and check the response body and status code (204 no content)

### /photos
- GET all photos and check all the response body with looping and status code (200 ok)
- GET photos by one id and check the response body and status code (200 ok)
- GET photos by 10 id with looping and check the response body and status code (200 ok)
- POST photos with request body and check the response and status code (201 created)
- DELETE photos with id and check the response body and status code (204 no content)

### /todos
- GET all todos and check all the response body with looping and status code (200 ok)
- GET todos by one id and check the response body and status code (200 ok)
- GET todos by 10 id with looping and check the response body and status code (200 ok)
- POST todos with request body and check the response and status code (201 created)
- DELETE todos with id and check the response body and status code (204 no content)

### /users
- GET all users and check all the response body with looping and status code (200 ok)
- GET users by one id and check the response body and status code (200 ok)
- GET users by 10 id with looping and check the response body and status code (200 ok)
- POST users with request body and check the response and status code (201 created)
- DELETE users with id and check the response body and status code (204 no content)

## Summary
All Test Case except for DELETE method is passed. DELETE test case is failed because the service return 200 status code.



