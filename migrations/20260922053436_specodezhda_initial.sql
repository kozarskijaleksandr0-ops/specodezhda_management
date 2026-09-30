CREATE TABLE IF NOT EXISTS specodezhda (
    id              BLOB PRIMARY KEY NOT NULL,   
    vid             TEXT NOT NULL,              
    srok_noski      INTEGER NOT NULL,          
    stoimost        REAL NOT NULL,              
    created_at      TEXT NOT NULL,               
    updated_at      TEXT,                       
    deleted_at      TEXT                         
);