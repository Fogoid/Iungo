package server

import (
	"fmt"
	"log"
	"net/http"

	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
)

func (s *Server) RegisterRoutes() http.Handler {
	e := echo.New()
	e.Use(middleware.Logger())
	e.Use(middleware.Recover())

	e.GET("/health", s.healthHandler)
	e.POST("/", s.CreatePostHandler)

	return e
}

func (s *Server) healthHandler(c echo.Context) error {
	return c.JSON(http.StatusOK, s.db.Health())
}

type Visibility string

const (
    PUBLIC Visibility = "PUBLIC"
    PRIVATE Visibility = "PRIVATE"
)

type Post struct {
    Author string `json:"author"`
    PostBody string `json:"post_body"`
    Visibility Visibility `json:"visibility"`
}

func (p *Post) Validate() error {
    if p.Author == "" {
        return fmt.Errorf("Author must not be empty")
    } else if p.PostBody == "" {
        return fmt.Errorf("PostBody must not be empty")
    } else if p.Visibility != PUBLIC && p.Visibility != PRIVATE {
        return fmt.Errorf("Visibility must be either PUBLIC or PRIVATE")
    }

    return nil;
}

func (s *Server) CreatePostHandler(c echo.Context) error {
    value := new(Post)
    err := c.Bind(&value)
    if err != nil {
        return c.JSON(http.StatusBadRequest, err)
    }

    log.Printf("PostBody: %s, Visibility: %s, Author: %s", value.PostBody, value.Visibility, value.Author)

    err = value.Validate()
    if err != nil {
         return c.JSON(http.StatusBadRequest, err.Error())
    }

    return c.JSON(http.StatusCreated, value)
}

