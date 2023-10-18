# A Solana Demo

This is a proof of concept for developing Solana programs using dddappp low-code tool.

This PoC version of dddappp tool is simple and has very limited support for the DDDML specification.

But with it, we can believe that if the Solana version of the dddappp CLI has the features that the Move version already has,
it can be an amazing improvement in development efficiency when developing certain applications.

## Prerequisites

Currently, the dddappp low-code tool is published as a Docker image for developers to experience.

So before getting started, you need to:

* Install [Rust & Anchor & Solana & Yarn](https://www.anchor-lang.com).
* Install [Docker](https://docs.docker.com/engine/install/).


## Programming

Use Anchor init a workspace:

```shell
anchor init a-solana-demo
cd a-solana-demo
```

### Write DDDML Model File

In the `dddml` directory in the root of the repository, create a DDDML file `blog.yaml` like this:

```yaml
aggregates:
  Tag:
    metadata:
      # The following two lines indicate that the tool should automatically generate the Create and Update methods,
      # but not the Delete method.
      Preprocessors: [ "MOVE_CRUD_IT" ]
      CRUD_IT_NO_DELETE: true
    id:
      name: Name
      type: String
      generator:
        class: assigned
    properties:
      Description:
        type: String
        length: 100

  Article:
    metadata:
      Preprocessors: [ "MOVE_CRUD_IT" ]
      CRUD_IT_NO_DELETE: true
    id:
      name: ArticleId
      type: u128
    properties:
      Title:
        type: String
        length: 200
      Body:
        type: String
        length: 2000
      Owner:
        type: address
      Comments:
        itemType: Comment
    entities:
      Comment:
        metadata:
          Preprocessors: [ "MOVE_CRUD_IT" ]
          CRUD_IT_NO_REMOVE_ENTITY: true
        id:
          name: CommentSeqId
          type: u64
        properties:
          Commenter:
            type: String
            length: 100
          Body:
            type: String
            length: 500
          Owner:
            type: address
    # -----------------------------------------------------
    # Methods of the Article aggregate.
    methods:
      UpdateArticleBody:
        parameters:
          Body:
            type: String
        event:
          name: ArticleBodyUpdated

  Blog:
    metadata:
      Preprocessors: [ "MOVE_CRUD_IT" ]
      CRUD_IT_NO_DELETE: true
    id:
      name: Owner
      type: address
      generator:
        class: signer
    properties:
      Name:
        type: String
        length: 200
```

You may have noticed the comments in the code above. 
If the "business logic" you need is CRUD, you don't actually need to write it. 
You can specify the tool to automatically generate it.

> **Tip**
>
> About DDDML, here is an introductory article: ["Introducing DDDML: The Key to Low-Code Development for Decentralized Applications"](https://github.com/wubuku/Dapp-LCDP-Demo/blob/main/IntroducingDDDML.md).
>
> This article describes some of the DDDML (DSL) specifications that are already supported in the current version of Move.

### Run dddappp Project Creation Tool

In repository root directory, run:

```shell
docker run \
-v .:/myapp \
wubuku/dddappp-solana:0.0.1 \
--dddmlDirectoryPath /myapp/dddml \
--boundedContextName Test.ASolanaDemo \
--solanaProgramProjectDirectoryPath /myapp/programs/a-solana-demo \
--solanaProgramId 9TFvj5xg3X7URQGE8PYVEtb2HZ9mrGXRiWqmsdRX6xS3 \
--solanaBoundedContextMod a_solana_demo \
--boundedContextJavaPackageName org.test.asolanademo \
--javaProjectsDirectoryPath /myapp/solana-java-service \
--javaProjectNamePrefix asolanademo \
--pomGroupId test.asolanademo
```

The command parameters above are straightforward:

* The first line indicates mounting your local directory into the `/myapp` directory inside the container.
* `dddmlDirectoryPath` is the directory where DDDML model files are located. It should be a readable directory path in the container.
* Interpret the value of (optional) parameter `boundedContextName` as the name of your application you want to develop. When there are multiple parts in your name, separate them with dots and use PascalCase naming style for each part. Bounded-context is a term in Domain-driven design (DDD) that refers to a specific problem domain scope that contains specific business boundaries, constraints, and language. If you don't understand this concept for now, it's not a big deal.
* `solanaProgramProjectDirectoryPath` is directory path where on-chain Solana program project is placed. It should be a readable and writable directory path in container.
* `solanaProgramId` is Solana program Id. You can find it in `Anchor.toml`. Note to replace `9TFvj5xg3X7URQGE8PYVEtb2HZ9mrGXRiWqmsdRX6xS3` with the actual one of your project.
* `solanaBoundedContextMod` is the name of bounded context module name in Solana program. It's recommended to use snake_case naming style.
* `boundedContextJavaPackageName` is Java package name of off-chain service. According to Java naming conventions, it should be all lowercase and parts should be separated by dots.
* `javaProjectsDirectoryPath` is directory path where off-chain service code is placed. Off-chain service consists of multiple modules (projects). It should be a readable and writable directory path in container.
* `javaProjectNamePrefix` is name prefix of each module of off-chain service. It's recommended to use an all-lowercase name.
* `pomGroupId` is GroupId of off-chain service. We use Maven as project management tool for off-chain service. It should be all lowercase and parts should be separated by dots.

Currently the tool only generates on-chain contracts, so Java-related parameters are not actually used.

> **Tip**
> 
> Since the dddappp v0.0.1 image is updated frequently, 
> if you have run this image before, 
> you may be required to manually delete the image and pull it again before `docker run`.
> You can use the shell commands in the "Tips" section below to do this.
> 

### Implementing Customized Business Logic

Open file `programs/a-solana-demo/src/article_update_article_body_logic.rs`, and implement the business logic of the `UpdateArticleBody` method.

What you need to do is actually fill in the contents of the `verify` and `mutate` functions.:

```rust
pub(crate) fn verify(
    body: String,
    article: &Account<Article>,
    ctx: &Context<UpdateArticleBody>,
) -> ArticleBodyUpdated {
    let _ = ctx;
    ArticleBodyUpdated {
        article_id: article.article_id.clone(),
        version: article.version,
        body,
    }
}

pub(crate) fn mutate(
    article_body_updated: &ArticleBodyUpdated,
    article: &mut Account<Article>,
) {
    article.body = article_body_updated.body.clone();
}
```

If you open other `*_logic.rs` files, you will find that the tool has generated the business logic for those entity `Create` and `Update` methods.

Now you can build and deploy your program.


## Test the project locally

If you want to test your project locally, you can use the following commands:

Run local single-node dev cluster:

```shell
solana-test-validator --reset
```

Build & deploy program:

```shell
anchor build && anchor deploy
```

Stream transaction logs in another terminal.:

```shell
solana logs --url localhost
```

### Tests

A TypeScript test file exists in this repository: `tests/a-solana-demo.ts`.

Run test:

```shell
anchor run test
```

## Tips

### Update dddappp Docker Image

Use the following commands to clean up old containers re-pull the image.

```shell
# If you have already run it, you may need to Clean Up Exited Docker Containers first
docker rm $(docker ps -aq --filter "ancestor=wubuku/dddappp-solana:0.0.1")
# remove the image
docker image rm wubuku/dddappp-solana:0.0.1
# pull the image
git pull wubuku/dddappp-solana:0.0.1
```

## Move Version Examples

###  Blog Example for Sui

See it [here](https://github.com/dddappp/sui-blog-example). It only requires 30 or so lines of code (all of which is a description of the domain model) to be written by the developer, and then generates a blog example that emulates [RoR Getting Started](https://guides.rubyonrails.org/getting_started.html) in one click, without requiring the developer to write a single line of other code.

### Blog Example for Rooch

Here is a Rooch version blog example: https://github.com/rooch-network/rooch/blob/main/examples/blog/README.md

Rooch's Getting Started article on developing a simple Blog (in a non-low-code/write-all-the-code-manually way):
https://rooch.network/zh-CN/docs/getting-started#41-创建-move-项目

The code to accompany the article is here: https://github.com/rooch-network/rooch/tree/main/examples/simple_blog

This example is actually a modification of our "Developing a blog using a low-code approach" example.
Specifically, they removed `Comment` which is an "Aggregate Internal Entity",
and only kept `Article` which is the "Aggregate Root Entity ".
(It doesn't matter if you can't understand the DDD concept of "aggregate " or something like that ...)
The reason for deleting the `Comment` entity, as I understand it, 
is probably because if they were to explain how to manually code the functions of adding comments/updating comments/deleting comments, 
then this "introductory" article would be too long and would scare away the newbies.

The code of our "Low-code developing a Rooch-based Blog Example" is also available in the Rooch official repository: https://github.com/rooch-network/rooch/blob/main/examples/blog

You may have noticed that the low-code version of the example project is "officially" called "blog", while the "Getting Started" article version is called "simple blog".

As you can see from the names, the version we developed using the low-code approach is more complex; however, the developer has a lot less to do.

### A More Complex Sui Demo

If you are interested, you can find a more complex Sui Demo here: ["A Sui Demo"](https://github.com/dddappp/A-Sui-Demo).

### Build a Crowdfunding dApp using Sui Move

A crowdfunding project for educational purposes: https://github.com/dddappp/sui-crowdfunding-example

Developed using the Sui Move version of dddappp. The development process and testing procedures are documented in the README of the repository. The development efficiency is scary. 😄

