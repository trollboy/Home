
<script>
	import { data } from '../lib/fullnav';


	export let current_nav = data;
</script>

<div class="lcars-app-container">

	<!-- HEADER ==================================================== -->


	<!-- SIDE MENU ================================================= -->

	<div id="left-menu" class="lcars-column start-space lcars-u-1 ">
		<div class="lcars-element lcars-blue-bell-bg button">{current_nav[0].title}</div>
		<div class="lcars-element-addition"> </div>

		{#each current_nav[0].sub as menu_item}
			<div class="left-rounded lcars-element lcars-lilac-bg button">{menu_item.title}</div>
		{/each}
		<div class="lcars-element-addition"> </div>
		<div class="lcars-element lcars-anakiwa-bg button">Search</div>
		<!-- FILLER -->
		<div class="lcars-bar lcars-u-1 lcars-blue-bell-bg"></div>
	</div>

	<!-- FOOTER ==================================================== -->

	<div id="footer" class="lcars-row">
		<!-- ELBOW -->
		<div class="lcars-elbow left-top lcars-blue-bell-bg"></div>
		<!-- BAR -->
		<div class="lcars-bar horizontal both-divider bottom lcars-danub-bg"></div>
		<!-- ROUNDED EDGE -->
<!--			<div class="lcars-bar horizontal right-end left-divider bottom lcars-blue-bell-bg"></div>-->
	</div>

	<div id="header" class="lcars-row header">
		<!-- ELBOW -->
		<div class="lcars-elbow left-bottom lcars-danub-bg"></div>
		<div class="lcars-column" style="margin-right: 0">
			<div class="lcars-bar horizontal both-right lcars-danub-bg"></div>

			<div class="lcars-row">
				<div class="lcars-column" style="float: left"><h1>{current_nav[0].title}</h1></div>
				<div class="lcars-column">fdfd</div>
			</div>
		</div>
		<div class="lcars-bar horizontal both-divider lcars-danub-bg right-align"><div class="lcars-title right">terminal.id</div></div>
	</div>
	<!-- MAIN CONTAINER -->
	<div id="container">
		<!-- COLUMN LAYOUT -->
		<div class="fill">

			<slot current_nav={current_nav}/>

		</div>

	</div>

</div>