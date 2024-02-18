DROP TABLE Humain;
CREATE TABLE Humain(id INT PRIMARY KEY, the_name VARCHAR(50), age INT, vivant BOOL DEFAULT true);
INSERT INTO Humain (id, the_name, age) VALUES (1, 'Joah', 20);
INSERT INTO Humain (id, the_name, age) VALUES (2, 'Martin', 19);
INSERT INTO Humain (id, the_name, age) VALUES (3, 'Raghid', 17);
INSERT INTO Humain (id, the_name, age) VALUES (4, 'Dabi', 18);
INSERT INTO Humain (id, the_name, age) VALUES (5, 'Vico', 18);
SELECT age, the_name FROM Humain WHERE age>18;