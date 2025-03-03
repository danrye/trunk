import cx from "classnames";
import styles from "./Hero.module.scss";
import Image from "next/image";
import ExtensionSearchBox from "../ExtensionSearchBox";
import { Inter } from "next/font/google";

const inter = Inter({ subsets: ["latin"], weight: ["400", "500", "800"] });

export default function Hero() {
  return (
    <section>
      <h1 className={cx(inter.className, styles.title)}>Expand Your Postgres Capabilities</h1>
      <div className={styles.searchContainer}>
        <ExtensionSearchBox></ExtensionSearchBox>
      </div>
      <h2 className={cx(inter.className, styles.subtitle)}>The easiest way to publish and install PostgreSQL extensions. </h2>
      <a href="https://github.com/CoreDB-io/trunk" className={styles.repoButton}>
        <Image src="/images/github.svg" alt="GitHub logo" width={20} height={20}></Image>
        <span className={cx(inter.className, styles.repoText)}>View on GitHub</span>
      </a>
      <p className={cx(inter.className, styles.body)}>
        Trunk is an open-source package installer and registry for PostgreSQL extensions. Use the Trunk CLI to easily publish and install
        PostgreSQL extensions and their dependencies.
      </p>
    </section>
  );
}
