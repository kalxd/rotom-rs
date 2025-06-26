create extension if not exists ltree;
create extension if not exists zhparser;

create table if not exists "用户" (
	编号 int primary key generated always as identity,
	用户名 varchar(64) not null unique,
	密码 text not null,
	创建日期 timestamptz not null default now()
);

create table if not exists "用户会话" (
	编号 int primary key generated always as identity,
	用户编号 int not null references "用户" ("编号"),
	记号 uuid not null unique,
	创建日期 timestamptz not null default now()
);
