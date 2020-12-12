name := "2020"

version := "0.1"

scalaVersion := "2.13.4"

val specs2Version     = "4.10.5"

libraryDependencies ++= Seq(
  "org.specs2"            %% "specs2-core"            % specs2Version  % "test",
  "org.specs2"            %% "specs2-matcher-extra"   % specs2Version  % "test",
  "org.specs2"            %% "specs2-junit"           % specs2Version  % "test",
)
