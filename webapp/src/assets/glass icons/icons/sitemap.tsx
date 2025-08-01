import React, {SVGProps} from 'react';

type IconProps = SVGProps<SVGSVGElement> & {
	secondaryfill?: string;
	strokewidth?: number;
	title?: string;
}

function Sitemap({fill = 'currentColor', secondaryfill, title = 'badge 13', ...props}: IconProps) {
	secondaryfill = secondaryfill || fill;

	return (
		<svg height="24" style={{}} width="24" {...props} viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
	<title>{title}</title>
	<g fill="none">
		<path d="M13 10H17C18.6569 10 20 11.3431 20 13V18C20 18.5523 19.5523 19 19 19C18.4477 19 18 18.5523 18 18V13C18 12.4477 17.5523 12 17 12H7C6.44772 12 6 12.4477 6 13V18C6 18.5523 5.55228 19 5 19C4.44772 19 4 18.5523 4 18V13C4 11.3431 5.34315 10 7 10H11V5.5C11 4.94772 11.4477 4.5 12 4.5C12.5523 4.5 13 4.94772 13 5.5V10Z" fill="url(#1752500502803-7223691_sitemap_existing_0_bmsfeqb4f)" mask="url(#1752500502803-7223691_sitemap_mask_dlr6lp1mr)" data-glass="origin"/>
		<path clipPath="url(#1752500502803-7223691_sitemap_clipPath_3gjnxoz2g)" d="M13 10H17C18.6569 10 20 11.3431 20 13V18C20 18.5523 19.5523 19 19 19C18.4477 19 18 18.5523 18 18V13C18 12.4477 17.5523 12 17 12H7C6.44772 12 6 12.4477 6 13V18C6 18.5523 5.55228 19 5 19C4.44772 19 4 18.5523 4 18V13C4 11.3431 5.34315 10 7 10H11V5.5C11 4.94772 11.4477 4.5 12 4.5C12.5523 4.5 13 4.94772 13 5.5V10Z" fill="url(#1752500502803-7223691_sitemap_existing_0_bmsfeqb4f)" data-glass="clone"/>
		<path d="M5 15C6.933 15 8.5 16.567 8.5 18.5C8.5 20.433 6.933 22 5 22C3.067 22 1.5 20.433 1.5 18.5C1.5 16.567 3.067 15 5 15ZM19 15C20.933 15 22.5 16.567 22.5 18.5C22.5 20.433 20.933 22 19 22C17.067 22 15.5 20.433 15.5 18.5C15.5 16.567 17.067 15 19 15ZM12 0.5C13.933 0.5 15.5 2.067 15.5 4C15.5 5.933 13.933 7.5 12 7.5C10.067 7.5 8.5 5.933 8.5 4C8.5 2.067 10.067 0.5 12 0.5Z" fill="url(#1752500502803-7223691_sitemap_existing_1_iwz8ajd78)" data-glass="blur"/>
		<path d="M14.75 4C14.75 2.48122 13.5188 1.25 12 1.25C10.4812 1.25 9.25 2.48122 9.25 4C9.25 5.51878 10.4812 6.75 12 6.75V7.5C10.067 7.5 8.5 5.933 8.5 4C8.5 2.067 10.067 0.5 12 0.5C13.933 0.5 15.5 2.067 15.5 4C15.5 5.933 13.933 7.5 12 7.5V6.75C13.5188 6.75 14.75 5.51878 14.75 4Z" fill="url(#1752500502803-7223691_sitemap_existing_4_epojfe8a8)"/>
		<path d="M7.75 18.5C7.75 16.9812 6.51878 15.75 5 15.75C3.48122 15.75 2.25 16.9812 2.25 18.5C2.25 20.0188 3.48122 21.25 5 21.25V22C3.067 22 1.5 20.433 1.5 18.5C1.5 16.567 3.067 15 5 15C6.933 15 8.5 16.567 8.5 18.5C8.5 20.433 6.933 22 5 22V21.25C6.51878 21.25 7.75 20.0188 7.75 18.5Z" fill="url(#1752500502803-7223691_sitemap_existing_5_rymtk9fgq)"/>
		<path d="M21.75 18.5C21.75 16.9812 20.5188 15.75 19 15.75C17.4812 15.75 16.25 16.9812 16.25 18.5C16.25 20.0188 17.4812 21.25 19 21.25V22C17.067 22 15.5 20.433 15.5 18.5C15.5 16.567 17.067 15 19 15C20.933 15 22.5 16.567 22.5 18.5C22.5 20.433 20.933 22 19 22V21.25C20.5188 21.25 21.75 20.0188 21.75 18.5Z" fill="url(#1752500502803-7223691_sitemap_existing_6_ltzhxtvlr)"/>
		<defs>
			<linearGradient id="1752500502803-7223691_sitemap_existing_0_bmsfeqb4f" gradientUnits="userSpaceOnUse" x1="12" x2="12" y1="4.5" y2="19">
				<stop stopColor="#575757"/>
				<stop offset="1" stopColor="#151515"/>
			</linearGradient>
			<linearGradient id="1752500502803-7223691_sitemap_existing_1_iwz8ajd78" gradientUnits="userSpaceOnUse" x1="12" x2="12" y1=".5" y2="22">
				<stop stopColor="#E3E3E5" stopOpacity=".6"/>
				<stop offset="1" stopColor="#BBBBC0" stopOpacity=".6"/>
			</linearGradient>
			<linearGradient id="1752500502803-7223691_sitemap_existing_2_jl7npcgr4" gradientUnits="userSpaceOnUse" x1="12" x2="12" y1=".5" y2="22">
				<stop stopColor="#E3E3E5" stopOpacity=".6"/>
				<stop offset="1" stopColor="#BBBBC0" stopOpacity=".6"/>
			</linearGradient>
			<linearGradient id="1752500502803-7223691_sitemap_existing_3_08432huzh" gradientUnits="userSpaceOnUse" x1="12" x2="12" y1=".5" y2="22">
				<stop stopColor="#E3E3E5" stopOpacity=".6"/>
				<stop offset="1" stopColor="#BBBBC0" stopOpacity=".6"/>
			</linearGradient>
			<linearGradient id="1752500502803-7223691_sitemap_existing_4_epojfe8a8" gradientUnits="userSpaceOnUse" x1="12" x2="12" y1=".5" y2="4.554">
				<stop stopColor="#fff"/>
				<stop offset="1" stopColor="#fff" stopOpacity="0"/>
			</linearGradient>
			<linearGradient id="1752500502803-7223691_sitemap_existing_5_rymtk9fgq" gradientUnits="userSpaceOnUse" x1="5" x2="5" y1="15" y2="19.054">
				<stop stopColor="#fff"/>
				<stop offset="1" stopColor="#fff" stopOpacity="0"/>
			</linearGradient>
			<linearGradient id="1752500502803-7223691_sitemap_existing_6_ltzhxtvlr" gradientUnits="userSpaceOnUse" x1="19" x2="19" y1="15" y2="19.054">
				<stop stopColor="#fff"/>
				<stop offset="1" stopColor="#fff" stopOpacity="0"/>
			</linearGradient>
			<clipPath id="1752500502803-7223691_sitemap_clipPath_3gjnxoz2g">
				<path d="M5 15C6.933 15 8.5 16.567 8.5 18.5C8.5 20.433 6.933 22 5 22C3.067 22 1.5 20.433 1.5 18.5C1.5 16.567 3.067 15 5 15ZM19 15C20.933 15 22.5 16.567 22.5 18.5C22.5 20.433 20.933 22 19 22C17.067 22 15.5 20.433 15.5 18.5C15.5 16.567 17.067 15 19 15ZM12 0.5C13.933 0.5 15.5 2.067 15.5 4C15.5 5.933 13.933 7.5 12 7.5C10.067 7.5 8.5 5.933 8.5 4C8.5 2.067 10.067 0.5 12 0.5Z" fill="url(#1752500502803-7223691_sitemap_existing_1_iwz8ajd78)"/>
			</clipPath>
			<mask id="1752500502803-7223691_sitemap_mask_dlr6lp1mr">
				<rect height="100%" width="100%" fill="#FFF"/>
				<path d="M5 15C6.933 15 8.5 16.567 8.5 18.5C8.5 20.433 6.933 22 5 22C3.067 22 1.5 20.433 1.5 18.5C1.5 16.567 3.067 15 5 15ZM19 15C20.933 15 22.5 16.567 22.5 18.5C22.5 20.433 20.933 22 19 22C17.067 22 15.5 20.433 15.5 18.5C15.5 16.567 17.067 15 19 15ZM12 0.5C13.933 0.5 15.5 2.067 15.5 4C15.5 5.933 13.933 7.5 12 7.5C10.067 7.5 8.5 5.933 8.5 4C8.5 2.067 10.067 0.5 12 0.5Z" fill="#000"/>
			</mask>
		</defs>
	</g>
</svg>
	);
};

export default Sitemap;