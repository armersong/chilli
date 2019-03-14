#!/bin/sh

curl -F "pic=@Cargo.lock; filename='Cargo.lock'" -H "Content-filename: Cargo.lock" http://localhost:5000/upload
