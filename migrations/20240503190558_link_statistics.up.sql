create table if not exists link_statistics
(
    id         serial primary key,
    link_id    text not null,
    referer    text,
    user_agent text,
    constraint fk_links
        foreign key (link_id)
            references links (id)
);

-- we will query on link_id more so we index it to make sure it stays and be as fast as possible
create index idx_link_statistics_link_id on link_statistics using btree (link_id);
