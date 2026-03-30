#import "@preview/typsidian:0.0.3": *

#show: typsidian.with(
  title: "Project Registration System", 
  course: "PIS",
  author: "Tým xkloub03"
)

#make-title()

#set page(
  paper: "a4",
  margin: (x: 2.2cm, y: 2.2cm),
)

#set text(
  lang: "en",
  size: 11pt,
)

#set par(
  justify: true,
  leading: 0.62em,
)

#set heading(numbering: "1.")

#let muted(body) = text(fill: rgb("#666"))[ #body ]
#let todo(body) = muted[*TODO: #body*]
#let field(name, value) = [*#name:* #value\ ]
#let secspace = v(0.85em)

#align(center)[
  #text(18pt, weight: "bold")[Information System Design]
  #v(0.35em)
  #text(14pt)[PIS 2026 — Microservices Architecture]
]

#v(1em)

#field("System name", "University Project Registration System")
#field("Architectural style", "Microservices")
#field("Author", "Tým xkloub03")
#field("Date", datetime.today().display("[day].[month].[year]"))

#secspace

= Introduction

This document describes the design of an information system for university project registration. The system is designed as a set of independent microservices communicating through gRPC contracts defined in `.proto` files. The frontend is a web application and the target frontend ↔ router boundary uses gRPC-Web.

== System goal

The goal of the system is to support:
- user registration and login,
- subject management,
- project management,
- student registration for subjects and projects,
- team management,
- notification delivery,
- project or team evaluation.

This system is intended primarily for teachers managing projects and for students participating in those projects. It provides students with a centralized platform to browse project topics, register, and track their participation, eliminating the need to rely on multiple sources such as email or external systems.

= User roles and use cases
#align(center)[
  #block(breakable: false)[
    #figure(
      image("usecase.svg"),
      caption: [System use-case diagram]
    ) <usecase>
  ]
]

== Roles

#table(
  columns: (2fr, 5fr),
  inset: 7pt,
  stroke: 0.6pt,
  [*Role*], [*Description*],
  [Student], [Registers, logs in, browses subjects and projects, enrolls, creates teams, submits solutions, checks notifications and evaluation results.],
  [Teacher], [Creates and manages projects, manages evaluations.],
  [Admin], [Manages users and subjects.],
)

= Conceptual data model
This section describes the data model of each microservice.

== Auth service


#figure(
  image("ERAuthService.png"),
  caption: [ER diagram for the Auth service],
)

== Notification service

#figure(
  image("ERNotificationService.png"),
  caption: [ER Diagram used in the Notification service]
) <er_notification>

In the notification record, `user_id` together with the notification’s own identifier forms a composite primary key. The `TO_BE_NOTIFIED_RECORD` timestamp is also used as a primary key for scheduled notification records, even though duplicate values may occur. These design decisions are discussed in @notification-service.


== Subject service
#align(center)[
  #figure(
    image("ERSubjectService.png", width: 50%),
    caption: [ER diagram of the Subject Service],
  )
]

== Project service
#align(center)[
  #figure(
    image("ERProjectService.png"),
    caption: [ER diagram of the Project Service],
  )
]

== Evaluation service
Evaluation service stores data in a key-value database, where @eval_data_model shows the two types of possible data stored as values. The DB has the following structure:
- *Key*
  - Value composed of a type discriminator and the ID of either project or subject evaluation.
- *Value*
  - Serialized data with the structure shown on @eval_data_model

#align(center)[
  #block(breakable: false)[
    #figure(
      image("evaluation.png", width: 90%),
      caption: [Diagram showing the two possible data types stored in the key-value DB]
    ) <eval_data_model>
  ]
]

= System architecture

The system is designed as a polyglot microservices architecture. Individual services are separated, independently deployable, and share only network contracts defined using ProtoBuf files, which are then used for gRPC communication

#align(center)[
  #block(breakable: false)[
    #figure(
      image("architecture.png"),
      caption: [Overview of the system architecture]
    )
  ]
]

== Service overview
Each service uses synchronous communication, except the Notification service, which uses asynchronous communication.

=== Frontend
A web application implemented in React with respect to the use case diagram in @usecase.

=== Router
The router is the system entry point. It routes requests from the frontend to internal services.

Responsibilities:
- single entry point
- identity and authorization checks
- calling internal gRPC services
- aggregating data for the frontend

=== Auth Service
The Auth Service is responsible for authentication and identity management within the system. Its primary functions include user registration, user login, JWT validation, and user management.

The service stores its data in #link("https://surrealdb.com/")[SurrealDB], a NoSQL database that supports both embedded and distributed deployment modes. In the context of this project, SurrealDB is used in embedded mode. This choice simplifies deployment while still allowing the service to be migrated to a distributed configuration in the future if scalability or operational requirements increase.

Responsibilities:
- registration of new users
- authentication of existing users
- validation of JWT tokens
- management of user accounts

=== Notification Service <notification-service>
The Notification Service is responsible for managing system notifications. It enables other services to create notifications for individual users or groups of users, either for immediate delivery or for delivery at a scheduled time.

Due to the nature of notification data, update and delete operations are expected to be minimal. For this reason, the service uses #link("https://github.com/fjall-rs/fjall")[Fjall] as its underlying database. Fjall is an embedded, log-structured key-value database, which is well suited for workloads characterized by frequent writes and infrequent modifications. Its log-structured design provides built-in crash recovery and efficient write performance.

The key-value model also supports the access patterns required by this service. Prefix-based queries make it possible to efficiently retrieve notifications for a particular user when the user identifier is stored as the leading part of the serialized key. In addition, ordered keys allow efficient querying of the next notification scheduled for delivery, as illustrated in @er_notification.

Responsibilities:
- creation of notifications
- retrieval of user notifications
- marking notifications as read
- delivery of notifications


=== Subject Service
Service for subject management.

Responsibilities:
- subject CRUD
- student enrollment into subjects
- validation of capacity / enrollment rules

=== Project Service
Service for project and team management.

Responsibilities:
- project CRUD
- student registration for projects
- team creation
- team membership management

=== Evaluation Service
Service for evaluation.

Responsibilities:
- create evaluation,
- update evaluation,
- retrieve evaluation for project / team / user,

== Technology

The following @tech_table showcase the technology will be used for this project

#figure(
  caption: [Technology stack],
  
table(
  columns: (2.2fr,  2.4fr, 2.1fr, 2fr),
  inset: 7pt,
  stroke: 0.6pt,
  [*Component*], [*Technology*], [*Communication*], [*Database*],
  [Frontend],  [React, Vite, Material UI], [gRPC-Web], [-],
  [Router],  [Rust, Tonic], [gRPC], [-],
  [Auth Service],  [Rust, Tonic], [gRPC], [SurrealDB],
  [Notification Service], [Rust, Tonic], [gRPC], [Fjall],
  [Subject Service],  [.NET], [gRPC], [#todo("fill in")],
  [Project Service],  [Java, Spring], [gRPC], [PostgreSQL],
  [Evaluation Service],  [Rust, Tonic], [gRPC], [FJall],
)) <tech_table>

== Sequence diagrams

#todo("Insert at least 2 sequence diagrams.")
#todo("Recommended scenarios:")
- user login,
- project registration,
#align(center)[
  #figure(
    image("Sequence_project_registration.png", width: 90%),
    caption: [Sequence Diagram of the Student Project Registration and Team Joining Workflow with emphasis on the asynchronously communicating notification service],
  )
]

- evaluation publication and notification creation.

= Security

Authentication and authorization are based on JSON Web Tokens (JWT) and are handled in cooperation with the Auth Service. After a successful login, the Auth Service issues a signed access token for the user. The same service is also responsible for validating the token during subsequent requests. A token is considered invalid if its digital sign does not match, expiration time has passed or if it has been revoked, for example after a logout operation.

== JWT Structure

Each token contains the following claims:

- `sub` — unique identifier of the authenticated user
- `role` — role assigned to the user, such as `Student`, `Teacher`, or `Admin`
- `exp` — expiration timestamp of the token

The token is signed using the symmetric HS256 algorithm.

== Authentication and Authorization Flow

Each request to a protected endpoint must include an access token in the `Authorization` header. After receiving such a request, the router extracts the token and forwards it to the Auth Service for validation. The Auth Service verifies the token signature, checks whether the token has expired, and determines whether it has been revoked. @auth illustrates the sequence of steps performed by the router when processing an authenticated request.


#figure(
  image("auth.png", width: 60%),
  caption: [Authorization flow]
) <auth>

= Team work distribution

#table(
  columns: (auto, auto, 1fr),
  inset: 7pt,
  stroke: 0.6pt,
  [*Team member*], [*xlogin*], [*Responsibility*],
  [Jakub Kloub], [xkloub03], [Evaluation service, Nix],
  [Jakub Jeřábek], [xjerab28], [Subject service],
  [Matúš Moravčík], [xmorav48], [Project service],
  [Le Duy Nguyen], [xnguye27], [Auth service, Notification service, Router, Frontend],
)