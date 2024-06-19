// scalac CaseClass.scala
// scala Main

// case class
// 1. default immutable fields
// 2. automatic to string, equls, and hashCode
// 3. pattern matching
// 4. copy method
// 5. companion object with apply method without new keyword and unapply for pattern matching
case class Employee(id: Int, name: String, stats: Vector[Int], jersey: Vector[Int]) // immutable fields as no var

object Main extends App {
  // init
  var e1 = Employee(3, "freeman", Vector(3, 4, 5), Vector(5)) // vector is immutable

  // update - vector is immutable but uses structural sharing to minimize copying when updates are made, by using tree like structure
  e1 = e1.copy(id = 1, name = "ohtani", stats = e1.stats.updated(0, 1).updated(1, 2).updated(2, 3) :+ 4, jersey = Vector(17))

  // print
  println(e1) // Employee(1, ohtani, Vector(1, 2, 3, 4), Vector(17))
}