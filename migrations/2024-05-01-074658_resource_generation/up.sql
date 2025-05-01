CREATE VIEW resource_generation AS
SELECT pb.player_id,
       SUM(br.population)::bigint    as population,
       SUM(br.food)::bigint          as food,
       SUM(br.wood)::bigint          as wood,
       SUM(br.stone)::bigint         as stone,
       SUM(br.gold)::bigint          as gold,
       SUM(br.food_acc_cap)::bigint  as food_acc_cap,
       SUM(br.wood_acc_cap)::bigint  as wood_acc_cap,
       SUM(br.stone_acc_cap)::bigint as stone_acc_cap,
       SUM(br.gold_acc_cap)::bigint  as gold_acc_cap
FROM player_building pb
         LEFT JOIN public.building_resource br
                   ON pb.building_id = br.building_id
                       AND pb.level = br.building_level
GROUP BY pb.player_id;