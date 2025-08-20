select
null as "id",
'默认分类' as "name!",
count(编号) as "count!"
from 表情
where 分类编号 is null
union all
select
cat.编号 as "id", cat.名称 as "name", t.count as "count!"
from
分类 as cat,
lateral (
	select count(编号) as count
	from 表情 as emoji
	where emoji.分类编号 = cat.编号
) as t
where cat.用户编号 = $1
order by id desc
