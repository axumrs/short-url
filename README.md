# 使用AXUM构建短链接服务

[本专题](https://axum.rs/subject/short-url)将带你使用 axum 构建一个短链接服务。短链接，又称短网址，是指将一个可能比较长的链接变成固定长度的短链接，便于在社交媒体、短信等渠道发布。另外，它也可以隐藏原链接里的一些敏感信息，比如带邀请码的推广链接。

> 本专题假设你已经学完了《[漫游 axum](https://axum.rs/subject/roaming-axum)》和《[使用 axum 构建 todo 服务](https://axum.rs/subject/todo-service)》专题

## 目标

本专题的目标是实现一个带 UI 界面的短链接服务，具体的：

- 显示一个表单，用户通过表单将原始链接提交到我们的服务

- 将生成的短链接显示给用户

- 通过短链接跳转到原始链接

- 显示一个访问量排行页面

## 关键技术

- axum

- 模板

- PostgreSQL

- Bootstrap

本专题的源码位于[axumrs/short-url](https://github.com/axumrs/short-url/tree/main)，并且每一章都有独立分支。

