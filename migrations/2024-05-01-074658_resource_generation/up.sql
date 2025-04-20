CREATE VIEW resource_generation AS
SELECT ub.user_id,
       SUM(br.population)    as population,
       SUM(br.food)          as food,
       SUM(br.wood)          as wood,
       SUM(br.stone)         as stone,
       SUM(br.gold)          as gold,
       SUM(br.food_acc_cap)  as food_acc_cap,
       SUM(br.wood_acc_cap)  as wood_acc_cap,
       SUM(br.stone_acc_cap) as stone_acc_cap,
       SUM(br.gold_acc_cap)  as gold_acc_cap
FROM user_buildings ub
         LEFT JOIN public.building_resource br
                   ON ub.building_id = br.building_id
                       AND ub.level = br.building_level
GROUP BY ub.user_id;
