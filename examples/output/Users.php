<?php

class Users
{
    
    /**
     * @var int
     */
    public int $id;
    
    /**
     * @var string
     */
    public string $username;
    
    /**
     * @var string
     */
    public string $email;
    
    /**
     * @var \DateTime
     */
    public \DateTime $created_at;
    
    /**
     * @var ?string
     */
    public ?string $name;
    
}