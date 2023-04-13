<div style="text-align: center;">
  <img src="assets/svg.svg" alt="" style="max-width: 100%; height: auto;">
</div>

# FrogifyBackend

Frogify is a Social Media platform i made as a School project in 9th grade. Goal is to bring School life, Information and the Social aspect all together in one place. 

This is the Backend of the project. This involves the API and the Database.

## Endpoints:
- `/auth/login` 
- `/auth/register`
- `/user/chngpwd`
- `/user/chngusrn`
- `/user/getacc`
- `/user/delacc`
- `/post/createpost`
- `/post/deletepost`
- `/post/likepost`
- `/post/getpost`
- `/post/unlikepost`
- `/post/getallposts`
- `/info/weather`
- `/info/news`
- `/info/echo` (not really an echo more like a ping)


## Features
1. Auth:
    - Login
    - Register
    - Token login
   - Password reset
2. User:
    - change password
    - change username
    - get information's
    - delete account
3. Post:
   - Create
   - Delete
   - Like

## Future Features
1. Auth:
    - Email verification
    - 2FA
2. Posts:
    - Comment
    - Share
3. Profile pictures https://www.dicebear.com

## Known vulnerabilities
- Login as admin 
- Delete posts of other users

## Tools
To build the project you need to have the following installed:
- [Rust](https://www.rust-lang.org/tools/install)
- [MongoDB](https://docs.mongodb.com/manual/installation/)\
**Recommended:**
- [MongoDB Compass](https://www.mongodb.com/products/compass)
- [Postman](https://www.postman.com/downloads/)
