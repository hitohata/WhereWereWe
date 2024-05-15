# Travel

The travel represents a single travel.
This means the travel is not bound to a single spot. (Of course, it is possible.)

When you go on the honeymoon, you can go to Europe, then Asia, America, and so on.
In this case, the travel will be 'Honeymoon' instead of 'Europe', 'Asia'...

## Attributes

#### Travel ID

This is UUID

#### Name

Arbitrary string.
More than 0, and less than and equal to 100.

#### Start Date

Date with timezone

#### End Date

Data with timezone.
This can be null.
You can start a journey following your heart, and stop your journey when you find the place you want to live.

#### Travelers

The user ID collection.
This represents actual travelers.

#### Involved Users

The user ID collection.
This represents involved people like host.
