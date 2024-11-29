/*M!999999\- enable the sandbox mode */ 
-- MariaDB dump 10.19-11.6.2-MariaDB, for Linux (x86_64)
--
-- Host: localhost    Database: task_manager
-- ------------------------------------------------------
-- Server version	11.6.2-MariaDB

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*M!100616 SET @OLD_NOTE_VERBOSITY=@@NOTE_VERBOSITY, NOTE_VERBOSITY=0 */;

--
-- Table structure for table `categories`
--

DROP TABLE IF EXISTS `categories`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `categories` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `user_id` int(11) NOT NULL,
  `title` varchar(255) NOT NULL,
  `description` text DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `idx_category_title` (`title`)
) ENGINE=InnoDB AUTO_INCREMENT=294 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `categories`
--

LOCK TABLES `categories` WRITE;
/*!40000 ALTER TABLE `categories` DISABLE KEYS */;
INSERT INTO `categories` VALUES
(206,205,'sdadadasdsa',NULL),
(218,47,'Учеба',NULL),
(293,46,'sada',NULL);
/*!40000 ALTER TABLE `categories` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `comments`
--

DROP TABLE IF EXISTS `comments`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `comments` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `task_id` int(11) NOT NULL,
  `text` text NOT NULL,
  `created_at` timestamp NULL DEFAULT current_timestamp(),
  PRIMARY KEY (`id`),
  KEY `task_id` (`task_id`),
  CONSTRAINT `comments_ibfk_1` FOREIGN KEY (`task_id`) REFERENCES `tasks` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=376 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `comments`
--

LOCK TABLES `comments` WRITE;
/*!40000 ALTER TABLE `comments` DISABLE KEYS */;
/*!40000 ALTER TABLE `comments` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `tasks`
--

DROP TABLE IF EXISTS `tasks`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `tasks` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `user_id` int(11) NOT NULL,
  `title` varchar(255) NOT NULL,
  `status` varchar(32) DEFAULT NULL,
  `category_id` int(11) DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `user_id` (`user_id`),
  KEY `idx` (`user_id`,`title`),
  KEY `fk_category_id` (`category_id`),
  CONSTRAINT `fk_category_id` FOREIGN KEY (`category_id`) REFERENCES `categories` (`id`) ON DELETE CASCADE,
  CONSTRAINT `tasks_ibfk_1` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=723 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `tasks`
--

LOCK TABLES `tasks` WRITE;
/*!40000 ALTER TABLE `tasks` DISABLE KEYS */;
INSERT INTO `tasks` VALUES
(722,46,'da','Not Completed',293);
/*!40000 ALTER TABLE `tasks` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `users`
--

DROP TABLE IF EXISTS `users`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `users` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `username` varchar(32) NOT NULL,
  `password` varchar(256) NOT NULL,
  `email` varchar(64) DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=48 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `users`
--

LOCK TABLES `users` WRITE;
/*!40000 ALTER TABLE `users` DISABLE KEYS */;
INSERT INTO `users` VALUES
(46,'klewy','fishdom12','sasad'),
(47,'gandon','gandon','sdafdafadsd@gmail.com');
/*!40000 ALTER TABLE `users` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Temporary table structure for view `view_last_5_comments`
--

DROP TABLE IF EXISTS `view_last_5_comments`;
/*!50001 DROP VIEW IF EXISTS `view_last_5_comments`*/;
SET @saved_cs_client     = @@character_set_client;
SET character_set_client = utf8;
/*!50001 CREATE VIEW `view_last_5_comments` AS SELECT
 1 AS `text`,
  1 AS `created_at` */;
SET character_set_client = @saved_cs_client;

--
-- Temporary table structure for view `vw_active_users`
--

DROP TABLE IF EXISTS `vw_active_users`;
/*!50001 DROP VIEW IF EXISTS `vw_active_users`*/;
SET @saved_cs_client     = @@character_set_client;
SET character_set_client = utf8;
/*!50001 CREATE VIEW `vw_active_users` AS SELECT
 1 AS `username`,
  1 AS `task_count` */;
SET character_set_client = @saved_cs_client;

--
-- Temporary table structure for view `vw_category_task_count`
--

DROP TABLE IF EXISTS `vw_category_task_count`;
/*!50001 DROP VIEW IF EXISTS `vw_category_task_count`*/;
SET @saved_cs_client     = @@character_set_client;
SET character_set_client = utf8;
/*!50001 CREATE VIEW `vw_category_task_count` AS SELECT
 1 AS `category_title`,
  1 AS `task_cnt` */;
SET character_set_client = @saved_cs_client;

--
-- Temporary table structure for view `vw_empty_users`
--

DROP TABLE IF EXISTS `vw_empty_users`;
/*!50001 DROP VIEW IF EXISTS `vw_empty_users`*/;
SET @saved_cs_client     = @@character_set_client;
SET character_set_client = utf8;
/*!50001 CREATE VIEW `vw_empty_users` AS SELECT
 1 AS `username`,
  1 AS `email` */;
SET character_set_client = @saved_cs_client;

--
-- Temporary table structure for view `vw_recent_comments`
--

DROP TABLE IF EXISTS `vw_recent_comments`;
/*!50001 DROP VIEW IF EXISTS `vw_recent_comments`*/;
SET @saved_cs_client     = @@character_set_client;
SET character_set_client = utf8;
/*!50001 CREATE VIEW `vw_recent_comments` AS SELECT
 1 AS `task_id`,
  1 AS `text`,
  1 AS `created_at` */;
SET character_set_client = @saved_cs_client;

--
-- Temporary table structure for view `vw_task_cnt_by_category`
--

DROP TABLE IF EXISTS `vw_task_cnt_by_category`;
/*!50001 DROP VIEW IF EXISTS `vw_task_cnt_by_category`*/;
SET @saved_cs_client     = @@character_set_client;
SET character_set_client = utf8;
/*!50001 CREATE VIEW `vw_task_cnt_by_category` AS SELECT
 1 AS `user_id`,
  1 AS `category_title`,
  1 AS `task_cnt` */;
SET character_set_client = @saved_cs_client;

--
-- Temporary table structure for view `vw_tasks_with_categories`
--

DROP TABLE IF EXISTS `vw_tasks_with_categories`;
/*!50001 DROP VIEW IF EXISTS `vw_tasks_with_categories`*/;
SET @saved_cs_client     = @@character_set_client;
SET character_set_client = utf8;
/*!50001 CREATE VIEW `vw_tasks_with_categories` AS SELECT
 1 AS `task_id`,
  1 AS `task_title`,
  1 AS `category_title`,
  1 AS `username` */;
SET character_set_client = @saved_cs_client;

--
-- Temporary table structure for view `vw_tasks_with_comments`
--

DROP TABLE IF EXISTS `vw_tasks_with_comments`;
/*!50001 DROP VIEW IF EXISTS `vw_tasks_with_comments`*/;
SET @saved_cs_client     = @@character_set_client;
SET character_set_client = utf8;
/*!50001 CREATE VIEW `vw_tasks_with_comments` AS SELECT
 1 AS `task_title`,
  1 AS `comment_text`,
  1 AS `date` */;
SET character_set_client = @saved_cs_client;

--
-- Temporary table structure for view `vw_user_task_ratio`
--

DROP TABLE IF EXISTS `vw_user_task_ratio`;
/*!50001 DROP VIEW IF EXISTS `vw_user_task_ratio`*/;
SET @saved_cs_client     = @@character_set_client;
SET character_set_client = utf8;
/*!50001 CREATE VIEW `vw_user_task_ratio` AS SELECT
 1 AS `username`,
  1 AS `task_count`,
  1 AS `task_prcnt` */;
SET character_set_client = @saved_cs_client;

--
-- Temporary table structure for view `vw_users`
--

DROP TABLE IF EXISTS `vw_users`;
/*!50001 DROP VIEW IF EXISTS `vw_users`*/;
SET @saved_cs_client     = @@character_set_client;
SET character_set_client = utf8;
/*!50001 CREATE VIEW `vw_users` AS SELECT
 1 AS `username`,
  1 AS `email` */;
SET character_set_client = @saved_cs_client;

--
-- Final view structure for view `view_last_5_comments`
--

/*!50001 DROP VIEW IF EXISTS `view_last_5_comments`*/;
/*!50001 SET @saved_cs_client          = @@character_set_client */;
/*!50001 SET @saved_cs_results         = @@character_set_results */;
/*!50001 SET @saved_col_connection     = @@collation_connection */;
/*!50001 SET character_set_client      = utf8mb3 */;
/*!50001 SET character_set_results     = utf8mb3 */;
/*!50001 SET collation_connection      = utf8mb3_uca1400_ai_ci */;
/*!50001 CREATE ALGORITHM=UNDEFINED */
/*!50013 DEFINER=`klewy`@`localhost` SQL SECURITY DEFINER */
/*!50001 VIEW `view_last_5_comments` AS select `comments`.`text` AS `text`,`comments`.`created_at` AS `created_at` from `comments` order by `comments`.`created_at` desc limit 5 */;
/*!50001 SET character_set_client      = @saved_cs_client */;
/*!50001 SET character_set_results     = @saved_cs_results */;
/*!50001 SET collation_connection      = @saved_col_connection */;

--
-- Final view structure for view `vw_active_users`
--

/*!50001 DROP VIEW IF EXISTS `vw_active_users`*/;
/*!50001 SET @saved_cs_client          = @@character_set_client */;
/*!50001 SET @saved_cs_results         = @@character_set_results */;
/*!50001 SET @saved_col_connection     = @@collation_connection */;
/*!50001 SET character_set_client      = utf8mb3 */;
/*!50001 SET character_set_results     = utf8mb3 */;
/*!50001 SET collation_connection      = utf8mb3_uca1400_ai_ci */;
/*!50001 CREATE ALGORITHM=UNDEFINED */
/*!50013 DEFINER=`klewy`@`localhost` SQL SECURITY DEFINER */
/*!50001 VIEW `vw_active_users` AS select `users`.`username` AS `username`,count(`tasks`.`id`) AS `task_count` from (`users` join `tasks` on(`users`.`id` = `tasks`.`user_id`)) group by `users`.`id` order by count(`tasks`.`id`) desc */;
/*!50001 SET character_set_client      = @saved_cs_client */;
/*!50001 SET character_set_results     = @saved_cs_results */;
/*!50001 SET collation_connection      = @saved_col_connection */;

--
-- Final view structure for view `vw_category_task_count`
--

/*!50001 DROP VIEW IF EXISTS `vw_category_task_count`*/;
/*!50001 SET @saved_cs_client          = @@character_set_client */;
/*!50001 SET @saved_cs_results         = @@character_set_results */;
/*!50001 SET @saved_col_connection     = @@collation_connection */;
/*!50001 SET character_set_client      = utf8mb3 */;
/*!50001 SET character_set_results     = utf8mb3 */;
/*!50001 SET collation_connection      = utf8mb3_uca1400_ai_ci */;
/*!50001 CREATE ALGORITHM=UNDEFINED */
/*!50013 DEFINER=`klewy`@`localhost` SQL SECURITY DEFINER */
/*!50001 VIEW `vw_category_task_count` AS select `categories`.`title` AS `category_title`,count(`tasks`.`id`) AS `task_cnt` from (`categories` join `tasks` on(`categories`.`id` = `tasks`.`category_id`)) group by `categories`.`id` */;
/*!50001 SET character_set_client      = @saved_cs_client */;
/*!50001 SET character_set_results     = @saved_cs_results */;
/*!50001 SET collation_connection      = @saved_col_connection */;

--
-- Final view structure for view `vw_empty_users`
--

/*!50001 DROP VIEW IF EXISTS `vw_empty_users`*/;
/*!50001 SET @saved_cs_client          = @@character_set_client */;
/*!50001 SET @saved_cs_results         = @@character_set_results */;
/*!50001 SET @saved_col_connection     = @@collation_connection */;
/*!50001 SET character_set_client      = utf8mb3 */;
/*!50001 SET character_set_results     = utf8mb3 */;
/*!50001 SET collation_connection      = utf8mb3_uca1400_ai_ci */;
/*!50001 CREATE ALGORITHM=UNDEFINED */
/*!50013 DEFINER=`klewy`@`localhost` SQL SECURITY DEFINER */
/*!50001 VIEW `vw_empty_users` AS select `users`.`username` AS `username`,`users`.`email` AS `email` from `users` where !(`users`.`id` in (select `tasks`.`user_id` from `tasks`)) */;
/*!50001 SET character_set_client      = @saved_cs_client */;
/*!50001 SET character_set_results     = @saved_cs_results */;
/*!50001 SET collation_connection      = @saved_col_connection */;

--
-- Final view structure for view `vw_recent_comments`
--

/*!50001 DROP VIEW IF EXISTS `vw_recent_comments`*/;
/*!50001 SET @saved_cs_client          = @@character_set_client */;
/*!50001 SET @saved_cs_results         = @@character_set_results */;
/*!50001 SET @saved_col_connection     = @@collation_connection */;
/*!50001 SET character_set_client      = utf8mb3 */;
/*!50001 SET character_set_results     = utf8mb3 */;
/*!50001 SET collation_connection      = utf8mb3_uca1400_ai_ci */;
/*!50001 CREATE ALGORITHM=UNDEFINED */
/*!50013 DEFINER=`klewy`@`localhost` SQL SECURITY DEFINER */
/*!50001 VIEW `vw_recent_comments` AS select `comments`.`task_id` AS `task_id`,`comments`.`text` AS `text`,`comments`.`created_at` AS `created_at` from `comments` order by `comments`.`created_at` desc */;
/*!50001 SET character_set_client      = @saved_cs_client */;
/*!50001 SET character_set_results     = @saved_cs_results */;
/*!50001 SET collation_connection      = @saved_col_connection */;

--
-- Final view structure for view `vw_task_cnt_by_category`
--

/*!50001 DROP VIEW IF EXISTS `vw_task_cnt_by_category`*/;
/*!50001 SET @saved_cs_client          = @@character_set_client */;
/*!50001 SET @saved_cs_results         = @@character_set_results */;
/*!50001 SET @saved_col_connection     = @@collation_connection */;
/*!50001 SET character_set_client      = utf8mb3 */;
/*!50001 SET character_set_results     = utf8mb3 */;
/*!50001 SET collation_connection      = utf8mb3_uca1400_ai_ci */;
/*!50001 CREATE ALGORITHM=UNDEFINED */
/*!50013 DEFINER=`klewy`@`localhost` SQL SECURITY DEFINER */
/*!50001 VIEW `vw_task_cnt_by_category` AS select `tasks`.`user_id` AS `user_id`,`categories`.`title` AS `category_title`,count(`tasks`.`id`) AS `task_cnt` from (`categories` left join `tasks` on(`categories`.`id` = `tasks`.`category_id`)) group by `categories`.`title`,`tasks`.`user_id` */;
/*!50001 SET character_set_client      = @saved_cs_client */;
/*!50001 SET character_set_results     = @saved_cs_results */;
/*!50001 SET collation_connection      = @saved_col_connection */;

--
-- Final view structure for view `vw_tasks_with_categories`
--

/*!50001 DROP VIEW IF EXISTS `vw_tasks_with_categories`*/;
/*!50001 SET @saved_cs_client          = @@character_set_client */;
/*!50001 SET @saved_cs_results         = @@character_set_results */;
/*!50001 SET @saved_col_connection     = @@collation_connection */;
/*!50001 SET character_set_client      = utf8mb3 */;
/*!50001 SET character_set_results     = utf8mb3 */;
/*!50001 SET collation_connection      = utf8mb3_uca1400_ai_ci */;
/*!50001 CREATE ALGORITHM=UNDEFINED */
/*!50013 DEFINER=`klewy`@`localhost` SQL SECURITY DEFINER */
/*!50001 VIEW `vw_tasks_with_categories` AS select `t`.`id` AS `task_id`,`t`.`title` AS `task_title`,`c`.`title` AS `category_title`,`u`.`username` AS `username` from ((`tasks` `t` join `categories` `c` on(`t`.`category_id` = `c`.`id`)) join `users` `u` on(`t`.`user_id` = `u`.`id`)) */;
/*!50001 SET character_set_client      = @saved_cs_client */;
/*!50001 SET character_set_results     = @saved_cs_results */;
/*!50001 SET collation_connection      = @saved_col_connection */;

--
-- Final view structure for view `vw_tasks_with_comments`
--

/*!50001 DROP VIEW IF EXISTS `vw_tasks_with_comments`*/;
/*!50001 SET @saved_cs_client          = @@character_set_client */;
/*!50001 SET @saved_cs_results         = @@character_set_results */;
/*!50001 SET @saved_col_connection     = @@collation_connection */;
/*!50001 SET character_set_client      = utf8mb3 */;
/*!50001 SET character_set_results     = utf8mb3 */;
/*!50001 SET collation_connection      = utf8mb3_uca1400_ai_ci */;
/*!50001 CREATE ALGORITHM=UNDEFINED */
/*!50013 DEFINER=`klewy`@`localhost` SQL SECURITY DEFINER */
/*!50001 VIEW `vw_tasks_with_comments` AS select `tasks`.`title` AS `task_title`,`comments`.`text` AS `comment_text`,`comments`.`created_at` AS `date` from (`tasks` join `comments` on(`tasks`.`id` = `comments`.`task_id`)) */;
/*!50001 SET character_set_client      = @saved_cs_client */;
/*!50001 SET character_set_results     = @saved_cs_results */;
/*!50001 SET collation_connection      = @saved_col_connection */;

--
-- Final view structure for view `vw_user_task_ratio`
--

/*!50001 DROP VIEW IF EXISTS `vw_user_task_ratio`*/;
/*!50001 SET @saved_cs_client          = @@character_set_client */;
/*!50001 SET @saved_cs_results         = @@character_set_results */;
/*!50001 SET @saved_col_connection     = @@collation_connection */;
/*!50001 SET character_set_client      = utf8mb3 */;
/*!50001 SET character_set_results     = utf8mb3 */;
/*!50001 SET collation_connection      = utf8mb3_uca1400_ai_ci */;
/*!50001 CREATE ALGORITHM=UNDEFINED */
/*!50013 DEFINER=`klewy`@`localhost` SQL SECURITY DEFINER */
/*!50001 VIEW `vw_user_task_ratio` AS select `users`.`username` AS `username`,count(`tasks`.`id`) AS `task_count`,round(count(`tasks`.`id`) / (select count(0) from `tasks`) * 100,2) AS `task_prcnt` from (`users` left join `tasks` on(`users`.`id` = `tasks`.`user_id`)) group by `users`.`id` */;
/*!50001 SET character_set_client      = @saved_cs_client */;
/*!50001 SET character_set_results     = @saved_cs_results */;
/*!50001 SET collation_connection      = @saved_col_connection */;

--
-- Final view structure for view `vw_users`
--

/*!50001 DROP VIEW IF EXISTS `vw_users`*/;
/*!50001 SET @saved_cs_client          = @@character_set_client */;
/*!50001 SET @saved_cs_results         = @@character_set_results */;
/*!50001 SET @saved_col_connection     = @@collation_connection */;
/*!50001 SET character_set_client      = utf8mb3 */;
/*!50001 SET character_set_results     = utf8mb3 */;
/*!50001 SET collation_connection      = utf8mb3_uca1400_ai_ci */;
/*!50001 CREATE ALGORITHM=UNDEFINED */
/*!50013 DEFINER=`klewy`@`localhost` SQL SECURITY DEFINER */
/*!50001 VIEW `vw_users` AS select `users`.`username` AS `username`,`users`.`email` AS `email` from `users` */;
/*!50001 SET character_set_client      = @saved_cs_client */;
/*!50001 SET character_set_results     = @saved_cs_results */;
/*!50001 SET collation_connection      = @saved_col_connection */;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*M!100616 SET NOTE_VERBOSITY=@OLD_NOTE_VERBOSITY */;

-- Dump completed on 2024-11-29 21:34:37
