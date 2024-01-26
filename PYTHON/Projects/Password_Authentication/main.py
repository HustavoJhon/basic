import getpass
database = {"user": "root", "user1": "admin"}
username = input("Enter Yourname : ")
password = getpass.getpass("Enter Your Password : ")
for i in database.keys():
    if username == i:
        while password != database.get(i):
            password = getpass.getpass("Enter Your Password Again : ")
        break
print("Verified")
