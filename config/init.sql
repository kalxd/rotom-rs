create extension if not exists zhparser;

create text search configuration china (parser = zhparser);

ALTER TEXT SEARCH CONFIGURATION china ADD MAPPING FOR n,v,a,i,e,l WITH simple;

create table if not exists "用户" (
	编号 int primary key generated always as identity,
	用户名 varchar(64) not null unique,
	密码 text not null,
	创建日期 timestamptz not null default now()
);

create table if not exists "用户会话" (
	编号 int primary key generated always as identity,
	用户编号 int not null references "用户" ("编号"),
	令牌 uuid not null unique default gen_random_uuid(),
	创建日期 timestamptz not null default now()
);

create table if not exists "分类" (
	编号 int primary key generated always as identity,
	用户编号 int not null references 用户 (编号),
	名称 varchar(32) not null,
	创建日期 timestamptz not null default now()
);

create type file_type as enum ('png', 'jpg', 'webp');

create table if not exists "文件" (
	编号 int primary key generated always as identity,
	特征 text not null unique,
	扩展名 file_type not null,
	创建日期 timestamptz not null default now()
);

create table if not exists "表情" (
	编号 int primary key generated always as identity,
	分类编号 int references 分类 (编号),
	文件编号 int not null references 文件 (编号),
	描述 text,
	创建日期 timestamptz not null default now()
);
