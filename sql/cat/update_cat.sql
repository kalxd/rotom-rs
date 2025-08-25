with iq as (
    update
        分类
    set
        名称 = $3
    where
        用户编号 = $1
        and 编号 = $2
    returning
        *
)
select
    iq.编号 as "id", iq.名称 as "name", t.count as "count!"
from
    iq,
    lateral (
        select
            count(1)
        from
            表情
        where
            iq.编号 = 表情.分类编号) as t;

