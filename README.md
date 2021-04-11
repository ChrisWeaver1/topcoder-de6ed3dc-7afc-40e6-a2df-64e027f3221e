# [Topcoder challenge](https://www.topcoder.com/challenges/de6ed3dc-7afc-40e6-a2df-64e027f3221e)

Learning rust with a topcoder challenge

### Challenge Objectives

Create a web application in a language of your choice. What should it look like and what are the features? Check out the Individual Requirements section. It will have both a back end and a front end

### Project Background

This project involves an automated tester. The idea is to have members submit solutions and for the review to be carried out by another application that has been pre-configured to test solutions.

### Technology Stack

You will be creating both a front end and back end for the app. There is no restriction on the tech stack used for both front and back. No database is needed for this app 
Individual requirements

### Create a user interface in a language of your choice

- We’ll walk you through our expectation of how your application should work and you need to implement it to meet our expectations.
- The web application needs to launch on localhost at port 8080
- At route “/”, you need to prompt for basic authentication (the browser should prompt rather for basic auth - you will not be creating any login form or equivalent)
- You need to accept “topcoder” for username and “rocks” for password for the basic authentication
- Once the authentication is a success, you need to display a form to the user. The form will have an action of “/survey” and method of “post”
- Within this form, there needs to be an input of type “text”, id “name” and a maxlength of 20. The label for this input field will be “Name”
- Within this form, there needs to be an element of type “number”, id “age”, min value of 17 and max value of 40 (inclusive for both min and max values). The label for this input field will be “Age”
- Within this form, there needs to be a select element with id “favoriteFruit” and it’s default value should be “Banana”. The other values in the select element will be “Apple” and “Orange”. The label for this select will be “Favorite Fruit”
- Within this form, there needs to be a radio button group. The first radio element needs to have an id of “gender-male” (label will be Male) and the second radio button will have an id of “gender-female” (label will be Female). The radio button with id “gender-male” will be selected by default.
- Within this form, there needs to be a button of type “submit”. The text of the button will be “Submit”.
- We should be able to enter values into the inputs, select a value from the select dropdown, select a radio button in the group and click on the submit button.
- When we submit the form, you need to display a new page. The only content on this page will be a message  - “Thank you for completing the survey”
- That’s it!
- Once the application is ready, you need to prepare the application for deployment through docker. That is, you will need to submit a Dockerfile, along with your solution, which can build your solution and start a container which starts the application (where the application can be accessed at localhost:8080 inside the container)

### Important Notes

- The values mentioned above, for example, the value of the form’s action attribute is “post”, the default value in the select dropdown will be “Banana” etc - these values are case sensitive.
- Use the values exactly as mentioned. We are running automated tests and these are not perfect but if you follow the instructions along with the values exactly as mentioned, the tests should pass for your solution.
- There is a time limit (5 mins) for building the Dockerfile and launching the container for your solution. If your solution fails to build in time or your container fails to start in time, your solution will be rejected and scored 0.
- Your solution needs to follow a specific structure (folder structure). See What to Submit below

