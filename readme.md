<div style="text-align: center;">
  <img src="assets/FrogifyLogo.png" alt="" style="max-width: 100%; height: auto;">
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


## Features
1. Auth:
    - Login
    - Register
    - Token login
2. User:
    - change password
    - change username
    - get informations
    - delete account (posts also must be deleted but not implemented yet)

## Future Features
1. Auth:
    - Password reset
    - Email verification
    - 2FA
2. Posts:
    - Create
    - Delete
    - Edit
    - Like
    - Comment
    - Share
3. User:
    - Profile
    - Settings

## Tools
To build the project you need to have the following installed:
- [Rust](https://www.rust-lang.org/tools/install)
- [MongoDB](https://docs.mongodb.com/manual/installation/)\
**Recommended:**
- [MongoDB Compass](https://www.mongodb.com/products/compass)
- [Postman](https://www.postman.com/downloads/)
