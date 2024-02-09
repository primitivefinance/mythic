# Traditional MVC Architecture
Excaliburs user interface is designed to follow the Model-View-Controller architecture. 

![Traditional MVC Diagram](/assets/MVC.png)

- **Model:** This is the part of the system that handles data and business logic. It's responsible for retrieving data and converting it into meaningful concepts for your application. This could involve communicating with a database, processing data, or performing calculations.

- **View:** This is the part of the system that handles the presentation of data to the user. It's responsible for taking the data from the Model and displaying it in a format that the user can understand. This is often in the form of a user interface.
- **Controller:** This is the part of the system that handles user input. It's responsible for interpreting the user's actions, such as button clicks or key presses, and deciding what to do with them. This often involves updating the Model or the View.
- Model and view do not communication.
- Data flows in both the model and view is unidirectional.


## Hierarchical MVC

![Hierarchical MVC Diagram](/assets/HMVC.png)

This is what we have now. 
The main difference with the Hierarchical MVC is that there can be child MVCs and parent MVCs with one cononical root parant. 
The parent child communication happens between the component controllers. 
We are still exploring the pros and cons of this architecture.

**Pros:** 
- "modularization" of content structures. An example might be comments, ratings, Twitter or blog RSS feed displays. 
It is essentially a piece of content that needs to be displayed across multiple pages, and possibly even in different places

**cons:**
- HMVC can be more complex than traditional MVC, which can make it harder for beginners to understand and use. 
- HMVC can have a performance impact on your application because it requires more resources to load and execute code across multiple modules (we have not found this to be a problem yet)


## Middleware Pattern
The other catagory of MVC patterns we have considered is the middleware patterns. 
This is also what node.js does under the hood. 
One such middleware pattern we found compelling was the mediator pattern.
Differences between middleware and mediator is that the middleware pattern always outputs json while mediator outputs whatever we want.

![The Mediator Pattern](/assets/Mediator.png)